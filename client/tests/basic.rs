use env_logger::Env;

use easy_settings::SettingsMap;

use taple_client::{
    settings::{ClientSettings, SettingsGenerator},
    Client,
};

use taple_core::crypto::{Ed25519KeyPair, KeyGenerator, KeyMaterial};
use tempfile::tempdir;
use tokio::sync::oneshot;

#[test]
fn http_server_working() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut settings =
            ClientSettings::generate(&SettingsMap::new()).expect("Create ClientSettings");

        settings.http = true;
        settings.taple.node.secret_key = {
            let keypair = Ed25519KeyPair::from_seed(&[]);
            hex::encode(keypair.secret_key_bytes())
        };

        settings.db_path = {
            let db_tempdir = tempdir().unwrap();
            db_tempdir.path().to_str().unwrap().to_owned()
        };

        let client = Client::build(settings).expect("Client built");

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        client.bind_with_shutdown(shutdown_rx);

        tokio::spawn(async move {
            let response = reqwest::get("http://127.0.0.1:3000/api/subjects").await;
            shutdown_tx.send(()).unwrap();
            assert!(response.is_ok());
        });

        client.run(|_| {}).await;
    });
}
