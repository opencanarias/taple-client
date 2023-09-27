pub mod api;
pub mod doc;

pub use api::routes;
use std::{net::SocketAddr, sync::Arc};
use taple_core::{crypto::KeyPair, Api};
use tokio_util::sync::CancellationToken;
use utoipa::{openapi::OpenApiBuilder, OpenApi};
use utoipa_swagger_ui::Config;
use warp::{Filter, Rejection, Reply};

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
    extra_routes: Option<
        impl Filter<Extract = impl Reply, Error = Rejection> + Clone + Send + Sync + 'static,
    >,
    extra_open_api: Option<utoipa::openapi::OpenApi>,
) {
    let http_addr = format!("{}:{}", &settings.http_addr, &settings.http_port)
        .parse::<SocketAddr>()
        .unwrap();

    let client_api = http::api::routes(taple_api, keys, settings.taple.node.key_derivator);

    if settings.doc {
        let open_api = if extra_open_api.is_some() {
            extend_openapi_doc(ApiDoc::openapi(), extra_open_api.unwrap())
        } else {
            ApiDoc::openapi()
        };
        let openapi_json = warp::path!("doc" / "json")
            .and(warp::get())
            .map(move || warp::reply::json(&open_api));

        let swagger_ui = warp::path("doc")
            .and(warp::get())
            .and(warp::path::full())
            .and(warp::path::tail())
            .and(warp::any().map(move || Arc::new(Config::from("/doc/json"))))
            .and_then(serve_swagger);

        let routes = openapi_json.or(swagger_ui).or(client_api);

        if extra_routes.is_some() {
            let routes = routes.or(extra_routes.unwrap());
            let (_, server) =
                warp::serve(routes).bind_with_graceful_shutdown(http_addr, async move {
                    cancellation_token.cancelled().await;
                });
            tokio::spawn(server);
        } else {
            let (_, server) =
                warp::serve(routes).bind_with_graceful_shutdown(http_addr, async move {
                    cancellation_token.cancelled().await;
                });
            tokio::spawn(server);
        }
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

fn extend_openapi_doc(
    first_doc: utoipa::openapi::OpenApi,
    second_doc: utoipa::openapi::OpenApi,
) -> utoipa::openapi::OpenApi {
    let (servers, paths, components, tags, external_docs) = (
        first_doc.servers,
        first_doc.paths,
        first_doc.components,
        first_doc.tags,
        first_doc.external_docs,
    );
    let second_doc: OpenApiBuilder = second_doc.into();
    second_doc
        .paths(paths)
        .components(components)
        .tags(tags)
        .external_docs(external_docs)
        .servers(servers)
        .build()
}
