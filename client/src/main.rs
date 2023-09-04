extern crate env_logger;
mod database;
mod rest;
use database::leveldb::{open_db, LevelDBManager};
use log::info;
use rest::openapi::{serve_swagger, ApiDoc};
use std::sync::Arc;
use std::{error::Error, net::SocketAddr};
use taple_client::{client_settings_builder, ClientSettings, SettingsGenerator};
use taple_core::crypto::{Ed25519KeyPair, KeyGenerator, KeyPair, Secp256k1KeyPair};
use taple_core::{KeyDerivator, Taple, TapleShutdownManager, NodeAPI};
use tempfile::tempdir as tempdirf;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::RwLock;
use utoipa::OpenApi;
use warp::Filter;

#[tokio::main]
async fn main() {
    // Init logger
    env_logger::init();
    if let Err(error) = run().await {
        log::error!("{}", error);
    };
}

async fn run() -> Result<(), Box<dyn Error>> {
    let settings = ClientSettings::generate(&client_settings_builder().build())?;
    // debug!("{:?}", settings); // Look! includes private key

    // Open DATABASE DIR
    let tempdir;
    let path = if settings.database_path.is_empty() {
        tempdir = tempdirf().unwrap();
        tempdir.path().clone()
    } else {
        std::path::Path::new(&settings.database_path)
    };
    let db = open_db(path);
    let leveldb = LevelDBManager::new(db);
    let derivator = settings.taple.node.key_derivator.clone();
    let keys = if settings.taple.node.secret_key.is_some() {
        let current_key = settings.taple.node.secret_key.clone();
        let str_key = current_key.unwrap();
        match derivator {
            KeyDerivator::Ed25519 => KeyPair::Ed25519(Ed25519KeyPair::from_secret_key(
                &hex::decode(str_key).expect("Generate keypair from secret key"),
            )),
            KeyDerivator::Secp256k1 => KeyPair::Secp256k1(Secp256k1KeyPair::from_secret_key(
                &hex::decode(str_key).expect("Generate keypair from secret key"),
            )),
        }
    } else {
        panic!("No MC available");
    };
    ////////////////////
    let taple = Taple::new(settings.taple.clone(), leveldb);
    let api = taple.get_api();
    let shutdown_manager = taple.get_shutdown_manager();
    let signal_shutdown_manager = taple.get_shutdown_manager();
    let arc_taple = Arc::new(RwLock::new(Some(taple)));
    let arc_clone = arc_taple.clone();
    let mut stream = signal(SignalKind::terminate())?;
    tokio::task::spawn(async move {
        tokio::select! {
            _ = signal_shutdown_manager.wait_for_shutdown() => {

            },
            _ = stream.recv() => {
                let mut write_lock = arc_clone.write().await;
                let taple = write_lock.take();
                taple.unwrap().shutdown().await;
            }
        };
    });
    {
        let mut write_lock = arc_taple.write().await;
        let taple = write_lock.as_mut().unwrap();
        taple.start().await?;
        info!("Controller ID: {}", taple.controller_id().unwrap());
    }
    if settings.http {
        log::info!(
            "HTTP server listen on {}:{}",
            settings.http_addr,
            settings.http_port
        );
        start_http_server(settings, api, keys, derivator, shutdown_manager).await?;
    } else {
        log::warn!("HTTP server not enabled");
        shutdown_manager.wait_for_shutdown().await;
    }
    Ok(())
}

async fn start_http_server(
    settings: ClientSettings,
    taple: NodeAPI,
    keys: KeyPair,
    derivator: KeyDerivator,
    shutdown_manager: TapleShutdownManager,
) -> Result<(), Box<dyn Error>> {
    let swaggerui = settings.doc_ui.clone();
    let http_addr = format!("{}:{}", settings.http_addr, settings.http_port)
        .parse::<SocketAddr>()
        .unwrap();

    let config = Arc::new(utoipa_swagger_ui::Config::from("/api/doc/json"));

    let api_doc = warp::path!("api" / "doc" / "json")
        .and(warp::get())
        .map(|| warp::reply::json(&ApiDoc::openapi()));

    if swaggerui {
        log::warn!("DOC SERVER ACTIVATED");
        let swagger_ui = warp::path("api")
            .and(warp::path("doc"))
            .and(warp::get())
            .and(warp::path::full())
            .and(warp::path::tail())
            .and(warp::any().map(move || config.clone()))
            .and_then(serve_swagger);
        warp::serve(api_doc.or(swagger_ui).or(rest::routes::routes(
            taple,
            keys,
            derivator,
        )))
        .bind_with_graceful_shutdown(http_addr, async move {
            shutdown_manager.wait_for_shutdown().await
        })
        .1
        .await;
    } else {
        warp::serve(api_doc.or(rest::routes::routes(taple, keys, derivator)))
            .bind_with_graceful_shutdown(http_addr, async move {
                shutdown_manager.wait_for_shutdown().await
            })
            .1
            .await;
    }
    Ok(())
}
