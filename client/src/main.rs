extern crate env_logger;
mod database;
mod rest;
use database::leveldb::{open_db, LevelDBManager};
use leveldb::iterator::Iterable;
use log::info;
use rest::openapi::{serve_swagger, ApiDoc};
use std::sync::Arc;
use std::{error::Error, net::SocketAddr};
use taple_client::{client_settings_builder, ClientSettings, SettingsGenerator};
use taple_core::crypto::{Ed25519KeyPair, KeyGenerator, KeyPair, Secp256k1KeyPair};
use taple_core::{KeyDerivator, Taple};
use tempfile::tempdir as tempdirf;
use tokio::signal::unix::{signal, SignalKind};
use utoipa::OpenApi;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Init logger
    env_logger::init();
    let settings = ClientSettings::generate(&client_settings_builder().build())?;
    let dev_mode = settings.taple.node.dev_mode;
    let swaggerui = settings.swagger_ui.clone();
    if dev_mode {
        info!("DEV MODE is enabled. This is not a proper mode for production apps");
    }
    info!("{:?}", settings);
    // Open DATABASE DIR
    let tempdir;
    let path = if settings.taple.database.path.is_empty() {
        tempdir = tempdirf().unwrap();
        tempdir.path().clone()
    } else {
        std::path::Path::new(&settings.taple.database.path)
    };
    let db = open_db(path);
    let iter = db.iter(leveldb::options::ReadOptions::new());
    for i in iter {
        log::warn!("{}", i.0 .0)
    }
    let leveldb = LevelDBManager::new(db);
    if settings.taple.node.secret_key.is_some() && settings.taple.node.seed.is_some() {
        panic!("You can't set both secret_key and seed");
    }
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
    } else if settings.taple.node.seed.is_some() {
        let seed = settings.taple.node.seed.clone().unwrap();
        match derivator {
            KeyDerivator::Ed25519 => KeyPair::Ed25519(Ed25519KeyPair::from_seed(seed.as_bytes())),
            KeyDerivator::Secp256k1 => {
                KeyPair::Secp256k1(Secp256k1KeyPair::from_seed(seed.as_bytes()))
            }
        }
    } else {
        panic!("No MC available");
    };
    ////////////////////
    let mut taple = Taple::new(settings.taple.clone(), leveldb);
    taple.start().await?;
    info!("Controller ID: {}", taple.controller_id().unwrap());
    let http_addr = format!("{}:{}", settings.http_addr, settings.http_port)
        .parse::<SocketAddr>()
        .unwrap();
    let mut stream = signal(SignalKind::terminate())?;
    let config = Arc::new(utoipa_swagger_ui::Config::from("/api/doc/json"));

    let api_doc = warp::path!("api" / "doc" / "json")
        .and(warp::get())
        .map(|| warp::reply::json(&ApiDoc::openapi()));

    if swaggerui {
        let swagger_ui = warp::path("api")
            .and(warp::path("doc"))
            .and(warp::path("ui"))
            .and(warp::get())
            .and(warp::path::full())
            .and(warp::path::tail())
            .and(warp::any().map(move || config.clone()))
            .and_then(serve_swagger);
        warp::serve(
            api_doc
                .or(swagger_ui)
                .or(rest::routes::routes(taple.get_api(), keys)),
        )
        .bind_with_graceful_shutdown(http_addr, async move {
            stream.recv().await;
        })
        .1
        .await;
    } else {
        warp::serve(api_doc.or(rest::routes::routes(taple.get_api(), keys)))
            .bind_with_graceful_shutdown(http_addr, async move {
                stream.recv().await;
            })
            .1
            .await;
    }
    Ok(())
}
