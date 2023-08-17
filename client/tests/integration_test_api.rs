use log::info;
use settings::SettingsMap;
use std::{error::Error, net::SocketAddr, time::Duration};
use taple_client::{
    leveldb::{open_db, LDBCollection, LevelDBManager},
    routes,
};
use taple_client::{ApiDoc, ClientSettings, SettingsGenerator};
use taple_core::{
    crypto::{Ed25519KeyPair, KeyGenerator, KeyMaterial, KeyPair, Secp256k1KeyPair},
    KeyDerivator, Taple, TapleShutdownManager,
};
use tokio::signal::unix::{signal, SignalKind};
use utoipa::OpenApi;
use warp::Filter;

use tempfile::tempdir;

#[test]
fn basic_example() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut settings =
            ClientSettings::generate(&SettingsMap::new()).expect("Create ClientSettings");
        let keypair = Ed25519KeyPair::from_seed(&[]);
        let hex_private_key = hex::encode(&keypair.secret_key_bytes());
        settings.http = true;
        settings.taple.node.secret_key = Some(hex_private_key);

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

        settings.database_path = {
            let db_tempdir = tempdir().unwrap();
            db_tempdir.path().to_str().unwrap().to_owned()
        };

        let path = std::path::Path::new(&settings.database_path);
        let db = open_db(path);
        let leveldb = LevelDBManager::new(db);

        let mut taple = Taple::new(settings.taple.clone(), leveldb);
        let shutdown_manager = taple.get_shutdown_manager();
        let signal_shutdown_manager = taple.get_shutdown_manager();
        let mut stream = signal(SignalKind::terminate()).expect("Signal terminate");
        tokio::task::spawn(async move {
            let mut inner_receiver = signal_shutdown_manager.get_raw_receiver();
            tokio::select! {
                _ = inner_receiver.recv() => {

                },
                _ = stream.recv() => {
                    signal_shutdown_manager.shutdown().await;
                }
            };
        });
        taple.start().await.expect("TAPLE started");
        info!("Controller ID: {}", taple.controller_id().unwrap());
        tokio::time::sleep(Duration::from_secs(10)).await;

        if settings.http {
            log::info!(
                "HTTP server listen on {}:{}",
                settings.http_addr,
                settings.http_port
            );

            start_http_server(settings, taple, keys, derivator, shutdown_manager)
                .await
                .expect("Start server");
        } else {
            log::warn!("HTTP server not enabled");
            shutdown_manager.wait_for_shutdown().await;
        }

        let response = ureq::post("http://127.0.0.1:3000/api/event-requests")
            .send_json(ureq::json!({
              "request": {
                "Create": {
                  "governance_id": "",
                  "schema_id": "governance",
                  "namespace": "",
                  "name": "test"
                }
              }
            }))
            .expect("Create governance");
        info!("Create governance: {}", response.into_string().unwrap());

        tokio::time::sleep(Duration::from_secs(10)).await;

        let response = ureq::get("http://127.0.0.1:3000/api/subjects?subject_type=governances")
            .call()
            .expect("Get governance");
        info!("Get governances: {}", response.into_string().unwrap());
    });
}

async fn start_http_server(
    settings: ClientSettings,
    taple: Taple<LevelDBManager, LDBCollection>,
    keys: KeyPair,
    derivator: KeyDerivator,
    shutdown_manager: TapleShutdownManager,
) -> Result<(), Box<dyn Error>> {
    let http_addr = format!("{}:{}", settings.http_addr, settings.http_port)
        .parse::<SocketAddr>()
        .unwrap();

    let api_doc = warp::path!("api" / "doc" / "json")
        .and(warp::get())
        .map(|| warp::reply::json(&ApiDoc::openapi()));

    tokio::task::spawn(async move {
        warp::serve(api_doc.or(routes(taple.get_api(), keys, derivator)))
            .bind_with_graceful_shutdown(http_addr, async move {
                shutdown_manager.wait_for_shutdown().await
            })
            .1
            .await;
    });
    Ok(())
}
