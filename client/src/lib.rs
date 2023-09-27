pub mod database;
mod http;
pub mod settings;
mod taple;

use ::futures::Future;
use database::leveldb::{LDBCollection, LevelDBManager};
use settings::ClientSettings;
use taple_core::Api;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

use std::error::Error;

use taple_core::{Node, Notification};
use tokio_util::sync::CancellationToken;

use database::leveldb;

pub const EMPTY: Option<BoxedFilter<(String,)>> = None;

pub struct ClientBuilder {
    modules: Vec<Box<dyn FnOnce(Api, CancellationToken) -> Result<(), Box<dyn Error>>>>,
}

impl ClientBuilder {
    pub fn add_modules<F>(&mut self, closure: F)
    where
        F: FnOnce(Api, CancellationToken) -> Result<(), Box<dyn Error>> + 'static,
    {
        self.modules.push(Box::new(closure));
    }

    pub fn new() -> Self {
        ClientBuilder {
            modules: Vec::new(),
        }
    }

    pub fn build(
        self,
        settings: ClientSettings,
        extra_routes: Option<
            impl Filter<Extract = impl Reply, Error = Rejection> + Clone + Send + Sync + 'static,
        >,
        extra_open_api: Option<utoipa::openapi::OpenApi>,
    ) -> Result<Client, Box<dyn Error>> {
        let cancellation_token = CancellationToken::new();

        let (taple_node, taple_api, keys) = taple::build(&settings, cancellation_token.clone())?;

        for module in self.modules {
            if let Err(error) = module(taple_api.clone(), cancellation_token.clone()) {
                cancellation_token.cancel();
                log::error!("Initialization of additional module failed: {}", error);
                return Err(error);
            }
        }

        if settings.http {
            http::build(
                settings,
                taple_api,
                keys,
                cancellation_token.clone(),
                extra_routes,
                extra_open_api,
            );
        }

        Ok(Client {
            taple_node,
            cancellation_token,
        })
    }
}

pub struct Client {
    taple_node: Node<LevelDBManager, LDBCollection>,
    cancellation_token: CancellationToken,
}

impl Client {
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
