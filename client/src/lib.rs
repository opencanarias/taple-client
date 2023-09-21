mod http;
pub mod settings;
mod taple;

use ::futures::Future;
use settings::ClientSettings;

use std::error::Error;

use taple_core::{Node, Notification};
use tokio_util::sync::CancellationToken;

use taple_db_leveldb::leveldb::{LDBCollection, LevelDBManager};

pub struct Client {
    taple_node: Node<LevelDBManager, LDBCollection>,
    cancellation_token: CancellationToken,
}

impl Client {
    pub fn build(settings: ClientSettings) -> Result<Self, Box<dyn Error>> {
        let cancellation_token = CancellationToken::new();

        let (taple_node, taple_api, keys) = taple::build(&settings, cancellation_token.clone())?;

        if settings.http {
            http::build(settings, taple_api, keys, cancellation_token.clone());
        }

        Ok(Client {
            taple_node,
            cancellation_token,
        })
    }

    pub fn bind_with_shutdown(&self, shutdown_signal: impl Future + Send + 'static) {
        let cancellation_token = self.cancellation_token.clone();
        tokio::spawn(async move {
            shutdown_signal.await;
            log::info!("Shutdown signal received");
            cancellation_token.cancel();
        });
    }

    pub async fn run<H>(self, notifications_handler: H)
    where
        H: Fn(Notification),
    {
        self.taple_node
            .handle_notifications(notifications_handler)
            .await;
        self.cancellation_token.cancel();
        log::info!("Stopped");
    }
}
