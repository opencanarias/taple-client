extern crate env_logger;
use clap::{Parser, ValueEnum};
use commons::models::approval_signature::{Acceptance, ApprovalResponse, ApprovalResponseContent};
use commons::models::event::Event;
use commons::models::event_content::{EventContent, Metadata};
use commons::models::event_request::{EventRequest, EventRequestType, RequestData};
use commons::models::signature::{Signature, SignatureContent};
use commons::models::state::SubjectData;
use commons::{
    config::TapleSettings,
    identifier::derive::{digest::DigestDerivator, KeyDerivator},
};
use config::Source;
use config::{builder::DefaultState, Config, ConfigBuilder, ConfigError, Environment, File};
use rest::bodys::{Payload, SignatureRequestContent};
use core::StateRequestBodyUpper;
use core::event_request::{CreateRequest, RequestPayload, StateRequest};
use core::{DatabaseSettings, NetworkSettings, NodeSettings, Taple, SignatureRequest, ExternalEventRequestBody};
use log::{debug, info};
use rest::bodys::{
    CreateRequestBody, EventRequestTypeBody, PostEventBody, PostEventRequestBody, PutVoteBody,
    StateRequestBody,
};
use rest::handlers::{
    __path_get_all_governances_handler, __path_get_all_subjects_handler, __path_get_event_handler,
    __path_get_event_properties_handler, __path_get_events_of_subject_handler,
    __path_get_governance_handler, __path_get_pending_requests_handler,
    __path_get_single_request_handler, __path_get_subject_handler,
    __path_post_event_request_handler,
    __path_put_approval_handler,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::{error::Error, net::SocketAddr};
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

#[derive(Parser, Default, Debug, Clone)]
#[clap(version, about = "Node for a TAPLE Network")]
struct Args {
    /// Port HTTP for the API REST
    #[arg(long("hp"))]
    httpport: Option<u32>,
    /// Listening ADDR for the API REST
    #[arg(long("ha"))]
    httpaddr: Option<String>,
    /// Port for the node to listen for protocol messages
    #[arg(short('p'), long)]
    p2pport: Option<u32>,
    /// Listening address for protocol messages
    #[arg(short('a'), long)]
    addr: Option<String>,
    /// List of accesspoinrt to use by the node. Each element is separated by ';'
    #[arg(long)]
    accesspoints: Option<Vec<String>>,
    /// Seed to use to generate the MC
    #[arg(short('s'), long)]
    seed: Option<String>,
    /// Secret Key in hexadecimal to import into the node
    #[arg(short('k'), long)]
    secretkey: Option<String>,
    /// Key derivator to use by the TAPLE
    #[arg(long("kd"), value_enum)]
    keyderivator: Option<KeyDerivatorEnum>,
    /// Digest derivator to use by the TAPLE
    #[arg(long("dd"), value_enum)]
    digestderivator: Option<DigestDerivatorEnum>,
    /// Replication factor to use by the node.
    #[arg(short('f'), long)]
    factor: Option<f64>,
    /// Time to wait fot each message sended
    #[arg(short('t'), long)]
    timeout: Option<u32>,
    /// API KEY for the api rest server
    #[arg(long("apikey"))]
    apikey: Option<String>,
    /// Path where to store the database
    #[arg(short('d'), long)]
    databasepath: Option<String>,
    /// Flag to activate the developer mode
    #[arg(short('m'), long)]
    devmode: bool,
    /// To vote to response to all vote request. It requires the dev mode enabled
    #[arg(short('v'), long)]
    passvotation: Option<PassVotation>,
    /// Flag to activate swagger-ui
    #[arg(long("ui"))]
    swaggerui: bool,
}

impl Source for Args {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<config::Map<String, config::Value>, ConfigError> {
        let mut map: HashMap<String, config::Value> = HashMap::new();
        if self.httpport.is_some() {
            map.insert("httpport".into(), self.httpport.clone().unwrap().into());
        }
        if self.httpaddr.is_some() {
            map.insert("httpaddr".into(), self.httpaddr.clone().unwrap().into());
        }
        map.insert("apikey".into(), self.apikey.clone().into());
        if self.swaggerui {
            map.insert("swaggerui".into(), self.swaggerui.into());
        }
        if self.p2pport.is_some() {
            map.insert(
                "network.p2pport".into(),
                self.p2pport.clone().unwrap().into(),
            );
        }
        if self.addr.is_some() {
            map.insert("network.addr".into(), self.addr.clone().unwrap().into());
        }
        if self.accesspoints.is_some() {
            map.insert(
                "network.accesspoints".into(),
                self.accesspoints.clone().unwrap().into(),
            );
        }
        if self.seed.is_some() {
            map.insert("node.seed".into(), self.seed.clone().unwrap().into());
        }
        if self.secretkey.is_some() {
            map.insert(
                "node.secretkey".into(),
                self.secretkey.clone().unwrap().into(),
            );
        }
        if self.keyderivator.is_some() {
            let derivator: KeyDerivator = self.keyderivator.clone().unwrap().into();
            map.insert("node.keyderivator".into(), derivator.into());
        }
        if self.digestderivator.is_some() {
            let derivator: DigestDerivator = self.digestderivator.clone().unwrap().into();
            map.insert("node.digestderivator".into(), derivator.into());
        }
        if self.factor.is_some() {
            map.insert(
                "node.replicationfactor".into(),
                self.factor.clone().unwrap().into(),
            );
        }
        if self.timeout.is_some() {
            map.insert("node.timeout".into(), self.timeout.clone().unwrap().into());
        }
        if self.databasepath.is_some() {
            map.insert(
                "database.path".into(),
                self.databasepath.clone().unwrap().into(),
            );
        }
        if self.devmode {
            map.insert("node.devmode".into(), self.devmode.into());
        }
        let passvotation = if self.passvotation.is_none() {
            0
        } else {
            match self.passvotation.as_ref().unwrap() {
                PassVotation::Dissabled => 0,
                PassVotation::AlwaysNo => 2,
                PassVotation::AlwaysYes => 1,
            }
        };
        map.insert("node.passvotation".into(), passvotation.into());
        Ok(map)
    }
}

#[derive(Parser, Clone, Debug, ValueEnum, Default)]
enum KeyDerivatorEnum {
    #[default]
    Ed25519,
    Secp256k1,
}

#[derive(Parser, Clone, Debug, ValueEnum, Default)]
enum PassVotation {
    #[default]
    Dissabled,
    AlwaysYes,
    AlwaysNo,
}

impl Into<KeyDerivator> for KeyDerivatorEnum {
    fn into(self) -> KeyDerivator {
        match self {
            KeyDerivatorEnum::Ed25519 => KeyDerivator::Ed25519,
            KeyDerivatorEnum::Secp256k1 => KeyDerivator::Secp256k1,
        }
    }
}

#[derive(Parser, Clone, Debug, ValueEnum, Default)]
pub enum DigestDerivatorEnum {
    #[default]
    Blake3_256,
    Blake3_512,
    SHA2_256,
    SHA2_512,
    SHA3_256,
    SHA3_512,
}

impl Into<DigestDerivator> for DigestDerivatorEnum {
    fn into(self) -> DigestDerivator {
        match self {
            DigestDerivatorEnum::Blake3_256 => DigestDerivator::Blake3_256,
            DigestDerivatorEnum::Blake3_512 => DigestDerivator::Blake3_512,
            DigestDerivatorEnum::SHA2_256 => DigestDerivator::SHA2_256,
            DigestDerivatorEnum::SHA2_512 => DigestDerivator::SHA2_512,
            DigestDerivatorEnum::SHA3_256 => DigestDerivator::SHA3_256,
            DigestDerivatorEnum::SHA3_512 => DigestDerivator::SHA3_512,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Init logger
    env_logger::init();
    let args = Args::parse();
    let dev_mode = args.devmode;
    let settings = load_settings_from_file(args)?;
    let api_key = settings.x_api_key.clone();
    let swaggerui = settings.swagger_ui.clone();
    if dev_mode {
        info!("DEV MODE is enabled. This is not a proper mode for production apps");
    }
    info!("{:?}", settings);
    let mut taple = Taple::new(settings.get_taple_settings());
    taple.start().await?;
    info!("Controller ID: {}", taple.controller_id().unwrap());
    let http_addr = format!("{}:{}", settings.http_addr, settings.http_port)
        .parse::<SocketAddr>()
        .unwrap();
    let mut stream = signal(SignalKind::terminate())?;
    let config = Arc::new(utoipa_swagger_ui::Config::from("/api/doc/json"));
    #[derive(OpenApi)]
    #[openapi(
        paths(get_single_request_handler, post_event_request_handler, get_subject_handler, 
            get_all_subjects_handler, get_events_of_subject_handler, get_event_handler, 
            get_event_properties_handler, get_pending_requests_handler,
            put_approval_handler, get_all_governances_handler, get_governance_handler
        ),
        components(
            schemas(StateRequestBodyUpper, StateRequestBody, SignatureRequest, SignatureRequestContent, PostEventBody, RequestPayload, CreateRequestBody, CreateRequest, StateRequest, EventRequestTypeBody, RequestData, SubjectData, Acceptance, ApprovalResponse, ApprovalResponseContent, EventRequest, Payload, PostEventRequestBody, PutVoteBody, Event, EventRequestType, Signature, EventContent, SignatureContent, EventRequest, Metadata, ExternalEventRequestBody)
        ),
        modifiers(&SecurityAddon),
        security(),
        tags(
            (name = "Subjects"),
            (name = "Events"),
            (name = "Requests"),
            (name = "Approvals"),
            (name = "Governances")
        )
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap(); // We can unwrap safely since there already is components registered.
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("x-api-key"))),
            )
        }
    }

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
                .or(rest::routes::routes(taple.get_api(), api_key)),
        )
        .bind_with_graceful_shutdown(http_addr, async move {
            stream.recv().await;
        })
        .1
        .await;
    } else {
        warp::serve(api_doc.or(rest::routes::routes(taple.get_api(), api_key)))
            .bind_with_graceful_shutdown(http_addr, async move {
                stream.recv().await;
            })
            .1
            .await;
    }
    Ok(())
}

async fn serve_swagger(
    full_path: FullPath,
    tail: Tail,
    config: Arc<utoipa_swagger_ui::Config<'static>>,
) -> Result<Box<dyn Reply + 'static>, Rejection> {
    if full_path.as_str() == "/api/doc/ui" {
        return Ok(Box::new(warp::redirect::found(Uri::from_static(
            "/api/doc/ui/",
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
struct AppSettings {
    pub network: NetworkSettings,
    pub node: NodeSettings,
    pub database: DatabaseSettings,
    // pub settings: Settings,
    #[serde(rename = "httpaddr")]
    pub http_addr: String,
    #[serde(rename = "httpport")]
    pub http_port: u32,
    #[serde(rename = "apikey")]
    pub x_api_key: Option<String>,
    #[serde(rename = "swaggerui")]
    pub swagger_ui: bool,
}

impl AppSettings {
    fn get_taple_settings(&self) -> TapleSettings {
        TapleSettings {
            network: self.network.clone(),
            node: self.node.clone(),
            database: self.database.clone(),
        }
    }
}

fn load_settings_from_file(args: Args) -> Result<AppSettings, ConfigError> {
    debug!(
        "ENVVAR: {:?}",
        Environment::with_prefix("TAPLE").separator("_")
    );
    let config = Config::builder();
    let config = set_default(config)?;
    let config = config.add_source(File::with_name("settings").required(false));
    let config = config.add_source(args);
    let config = config.add_source(Environment::default());
    let mut config = config.add_source(Environment::with_prefix("TAPLE").separator("_"));
    match std::env::var("TAPLE_NETWORK_KNOWNNODES") {
        Ok(value) => {
            let known_nodes: Vec<String> = value.split(';').map(|f| f.to_string()).collect();
            config = config.set_override("network.knownnodes", known_nodes)?;
        }
        Err(_) => {}
    };
    let config = config.build()?;
    Ok(config.try_deserialize().unwrap())
}

fn set_default(
    config: ConfigBuilder<DefaultState>,
) -> Result<ConfigBuilder<DefaultState>, ConfigError> {
    //Client Settings
    let config = config.set_default("httpport", 3000u32)?;
    let config = config.set_default("httpaddr", "0.0.0.0")?;
    let config = config.set_default("apikey", Option::<String>::None)?;
    let config = config.set_default("swaggerui", false)?;

    //Core settings
    let default_taple_settings = Taple::get_default_settings();
    let config = config.set_default("network.p2pport", default_taple_settings.network.p2p_port)?;
    let config = config.set_default("network.addr", default_taple_settings.network.addr)?;
    let config = config.set_default(
        "network.knownnodes",
        default_taple_settings.network.known_nodes,
    )?;
    let config = config.set_default(
        "node.keyderivator",
        default_taple_settings.node.key_derivator,
    )?;
    let config = config.set_default("node.secretkey", default_taple_settings.node.secret_key)?;
    let config = config.set_default("node.seed", default_taple_settings.node.seed)?;
    let config = config.set_default(
        "node.digestderivator",
        default_taple_settings.node.digest_derivator,
    )?;
    let config = config.set_default(
        "node.replicationfactor",
        default_taple_settings.node.replication_factor,
    )?;
    let config = config.set_default("node.timeout", default_taple_settings.node.timeout)?;
    let config = config.set_default(
        "node.passvotation",
        default_taple_settings.node.passvotation,
    )?;
    let config = config.set_default("node.devmode", default_taple_settings.node.dev_mode)?;
    let config = config.set_default("database.path", default_taple_settings.database.path)?;
    Ok(config)
}
