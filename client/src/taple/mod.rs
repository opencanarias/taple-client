use std::path::Path;

use taple_core::{crypto::KeyPair, Api, Node};
use taple_db_leveldb::leveldb::{open_db, LDBCollection, LevelDBManager};
use taple_network_libp2p::network::*;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::ClientSettings;

pub fn build(
    settings: &ClientSettings,
    cancellation_token: CancellationToken,
) -> Result<(Node<LevelDBManager, LDBCollection>, Api, KeyPair), taple_core::Error> {
    let db = {
        let db = open_db(Path::new(&settings.db_path));
        LevelDBManager::new(db)
    };

    let keys = {
        let derivator = &settings.taple.node.key_derivator;
        let secret_key = &settings.taple.node.secret_key;
        KeyPair::from_hex(derivator, secret_key).expect("Key derivated")
    };

    let (sender, _receiver) = mpsc::channel(10000);
    let (notification_tx, _notification_rx) = mpsc::channel(1000);
    let network = NetworkProcessor::new(
        settings.taple.network.listen_addr.clone(),
        network_access_points(&settings.taple.network.known_nodes).unwrap(),
        sender,
        keys.clone(),
        cancellation_token.clone(),
        notification_tx,
        external_addresses(&settings.taple.network.external_address).unwrap(),
    )
    .expect("Network created");

    let (taple_node, taple_api) = Node::build(settings.taple.clone(), network, db)?;

    taple_node.bind_with_shutdown(async move {
        cancellation_token.cancelled().await;
    });

    Ok((taple_node, taple_api, keys))
}
