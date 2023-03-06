use taple_core::{
    NodeAPI,
    DatabaseSettings, NetworkSettings, NodeSettings, Taple,
};
use taple_client::{Payload, PostEventBody};
extern crate env_logger;
use commons::models::event::Event;
use commons::models::event_content::{EventContent, Metadata};
use commons::models::event_request::{
    CreateRequest, EventRequest, EventRequestType, RequestPayload, StateRequest,
};
use commons::models::signature::{Signature, SignatureContent};
use commons::models::state::SubjectData;
use commons::{
    config::TapleSettings,
    identifier::derive::{digest::DigestDerivator, KeyDerivator},
};
use taple_client::handlers::{
    __path_get_all_subjects_handler, __path_get_event_handler, __path_get_event_properties_handler,
    __path_get_events_of_subject_handler,
    __path_get_subject_handler,
};
use serde::Deserialize;
use std::env;
use std::sync::Arc;
use std::{net::SocketAddr};
use tokio::signal::unix::{signal, SignalKind};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use warp::{
    http::Uri,
    hyper::{Response, StatusCode},
    path::{FullPath, Tail},
    Filter, Rejection, Reply,
};

pub struct NodeBuilderAPI {
    timeout: Option<u32>,
    replication_factor: Option<f64>,
    digest_derivator: Option<DigestDerivator>,
    key_derivator: Option<KeyDerivator>,
    database_path: Option<String>,
    p2p_port: Option<u32>,
    addr: Option<String>,
    seed: Option<String>,
    access_points: Option<Vec<String>>,
    http_addr: Option<String>,
    http_port: Option<u32>,
    pass_votation: Option<u32>,
    dev_mode: Option<bool>,
}

impl NodeBuilderAPI {
    pub fn new() -> Self {
        Self {
            timeout: None,
            replication_factor: None,
            digest_derivator: None,
            key_derivator: None,
            database_path: None,
            p2p_port: None,
            addr: None,
            access_points: None,
            http_addr: None,
            http_port: None,
            seed: None,
            pass_votation: None,
            dev_mode: None,
        }
    }

    #[allow(dead_code)]
    fn set_env<T: ToString>(&mut self, value: Option<T>, name: &str) {
        if value.is_some() {
            env::set_var(name, value.unwrap().to_string());
        }
    }

    #[allow(dead_code)]
    pub fn build(mut self) -> Taple {
        let settings = TapleSettings {
            network: NetworkSettings {
                p2p_port: self.p2p_port.unwrap_or(40000u32),
                addr: self.addr.unwrap_or("/ip4/127.0.0.1/tcp".into()),
                known_nodes: self.access_points.unwrap_or(vec![]),
            },
            node: NodeSettings {
                key_derivator: self.key_derivator.take().unwrap_or(KeyDerivator::Ed25519),
                secret_key: None,
                digest_derivator: self
                    .digest_derivator
                    .take()
                    .unwrap_or(DigestDerivator::Blake3_256),
                replication_factor: self.replication_factor.take().unwrap_or(25f64),
                timeout: self.timeout.take().unwrap_or(3000u32),
                seed: self.seed,
                passvotation: self.pass_votation.unwrap_or(0) as u8,
                dev_mode: self.dev_mode.take().unwrap_or(false),
            },
            database: DatabaseSettings {
                path: self.database_path.unwrap_or("".into()),
            },
        };
        Taple::new(settings)
    }

    pub async fn run_with_api(mut self) -> NodeAPI {
        let settings = TapleSettings {
            network: NetworkSettings {
                p2p_port: self.p2p_port.unwrap_or(40000u32),
                addr: self.addr.unwrap_or("/ip4/127.0.0.1/tcp".into()),
                known_nodes: self.access_points.unwrap_or(vec![]),
            },
            node: NodeSettings {
                key_derivator: self.key_derivator.take().unwrap_or(KeyDerivator::Ed25519),
                secret_key: None,
                digest_derivator: self
                    .digest_derivator
                    .take()
                    .unwrap_or(DigestDerivator::Blake3_256),
                replication_factor: self.replication_factor.take().unwrap_or(25f64),
                timeout: self.timeout.take().unwrap_or(3000u32),
                seed: self.seed,
                passvotation: self.pass_votation.unwrap_or(0) as u8,
                dev_mode: self.dev_mode.take().unwrap_or(false),
            },
            database: DatabaseSettings {
                path: self.database_path.unwrap_or("".into()),
            },
        };
        let mut taple = Taple::new(settings);
        taple.start().await.unwrap();
        let http_addr = format!(
            "{}:{}",
            self.http_addr.unwrap_or(format!("127.0.0.1")),
            self.http_port.unwrap_or(3000)
        )
        .parse::<SocketAddr>()
        .unwrap();
        let mut stream = signal(SignalKind::terminate()).unwrap();
        let config = Arc::new(utoipa_swagger_ui::Config::from("/api-doc.json"));

        #[derive(OpenApi)]
        #[openapi(
        paths(get_subject_handler, get_all_subjects_handler, get_events_of_subject_handler, get_event_handler, get_event_properties_handler),
        components(
            schemas(SubjectData, Payload, PostEventBody, Event, EventRequestType, Signature, EventContent, SignatureContent, EventRequest, Metadata, CreateRequest, StateRequest, RequestPayload)
        ),
        modifiers(&SecurityAddon),
        tags(
            (name = "todo", description = "Todo items management API")
        )
    )]
        struct ApiDoc;

        struct SecurityAddon;

        impl Modify for SecurityAddon {
            fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
                let components = openapi.components.as_mut().unwrap(); // We can unwrap safely since there already is components registered.
                components.add_security_scheme(
                    "api_key",
                    SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new(
                        "apikeyexamplevalue123",
                    ))),
                )
            }
        }

        let api_doc = warp::path("api-doc.json")
            .and(warp::get())
            .map(|| warp::reply::json(&ApiDoc::openapi()));

        let swagger_ui = warp::path("swagger-ui")
            .and(warp::get())
            .and(warp::path::full())
            .and(warp::path::tail())
            .and(warp::any().map(move || config.clone()))
            .and_then(serve_swagger);
        let api_rest = warp::serve(
            api_doc
                .or(swagger_ui)
                .or(taple_client::routes(taple.get_api(), None)),
        )
        .bind_with_graceful_shutdown(http_addr, async move {
            stream.recv().await;
        })
        .1;
        tokio::spawn(api_rest);
        taple.get_api()
    }

    #[allow(dead_code)]
    pub fn with_database_path(mut self, path: String) -> Self {
        self.database_path = Some(path);
        self
    }

    pub fn add_access_point(mut self, access_point: String) -> Self {
        if self.access_points.is_none() {
            self.access_points = Some(Vec::new());
        }
        let access_points = self.access_points.as_mut().unwrap();
        access_points.push(access_point);
        self
    }

    pub fn with_dev_mode(mut self, mode: bool) -> Self {
        self.dev_mode = Some(mode);
        self
    }

    pub fn with_p2p_port(mut self, port: u32) -> Self {
        self.p2p_port = Some(port);
        self
    }

    #[allow(dead_code)]
    pub fn with_addr(mut self, addr: String) -> Self {
        self.addr = Some(addr);
        self
    }

    pub fn with_timeout(mut self, timeout: u32) -> Self {
        self.timeout = Some(timeout);
        self
    }

    #[allow(dead_code)]
    pub fn with_replication_factor(mut self, replication_factor: f64) -> Self {
        self.replication_factor = Some(replication_factor);
        self
    }

    pub fn with_pass_votation(mut self, pass_votation: u32) -> Self {
        self.pass_votation = Some(pass_votation);
        self
    }

    #[allow(dead_code)]
    pub fn with_digest_derivator(mut self, derivator: DigestDerivator) -> Self {
        self.digest_derivator = Some(derivator);
        self
    }

    #[allow(dead_code)]
    pub fn with_key_derivator(mut self, derivator: KeyDerivator) -> Self {
        self.key_derivator = Some(derivator);
        self
    }

    #[allow(dead_code)]
    pub fn with_http_addr(mut self, http_addr: String) -> Self {
        self.http_addr = Some(http_addr);
        self
    }

    pub fn with_http_port(mut self, http_port: u32) -> Self {
        self.http_port = Some(http_port);
        self
    }

    pub fn with_seed(mut self, seed: String) -> Self {
        self.seed = Some(seed);
        self
    }
}

async fn serve_swagger(
    full_path: FullPath,
    tail: Tail,
    config: Arc<utoipa_swagger_ui::Config<'static>>,
) -> Result<Box<dyn Reply + 'static>, Rejection> {
    if full_path.as_str() == "/swagger-ui" {
        return Ok(Box::new(warp::redirect::found(Uri::from_static(
            "/swagger-ui/",
        ))));
    }

    let path = tail.as_str();
    match utoipa_swagger_ui::serve(path, config) {
        Ok(file) => {
            if let Some(file) = file {
                Ok(Box::new(
                    Response::builder()
                        .header("Content-Type", file.content_type)
                        .body(file.bytes),
                ))
            } else {
                Ok(Box::new(StatusCode::NOT_FOUND))
            }
        }
        Err(error) => Ok(Box::new(
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error.to_string()),
        )),
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppSettings {
    pub settings: TapleSettings,
    #[serde(rename = "httpaddr")]
    pub http_addr: String,
    #[serde(rename = "httpport")]
    pub http_port: u32,
}
