pub mod api;
pub mod doc;

pub use api::routes;
use std::{net::SocketAddr, sync::Arc};
use taple_core::{crypto::KeyPair, Api};
use tokio_util::sync::CancellationToken;
use utoipa::OpenApi;
use utoipa_swagger_ui::Config;
use warp::Filter;

use crate::{
    http::{
        self,
        doc::{serve_swagger, ApiDoc},
    },
    settings::ClientSettings,
};

pub fn build(
    settings: ClientSettings,
    taple_api: Api,
    keys: KeyPair,
    cancellation_token: CancellationToken,
) {
    let http_addr = format!("{}:{}", &settings.http_addr, &settings.http_port)
        .parse::<SocketAddr>()
        .unwrap();

    let client_api = http::api::routes(
        taple_api,
        keys,
        settings.subjects_key_derivator,
        settings.taple.node.digest_derivator,
    );

    if settings.doc {
        let openapi_json = warp::path!("doc" / "json")
            .and(warp::get())
            .map(|| warp::reply::json(&ApiDoc::openapi()));

        let swagger_ui = warp::path("doc")
            .and(warp::get())
            .and(warp::path::full())
            .and(warp::path::tail())
            .and(warp::any().map(move || Arc::new(Config::from("/doc/json"))))
            .and_then(serve_swagger);

        let routes = openapi_json.or(swagger_ui).or(client_api);

        let (_, server) = warp::serve(routes).bind_with_graceful_shutdown(http_addr, async move {
            cancellation_token.cancelled().await;
        });

        tokio::spawn(server);
    } else {
        let routes = client_api;

        let (_, server) = warp::serve(routes).bind_with_graceful_shutdown(http_addr, async move {
            cancellation_token.cancelled().await;
        });

        tokio::spawn(server);
    }

    log::info!(
        "HTTP server listen on {}:{}",
        settings.http_addr,
        settings.http_port
    );
}
