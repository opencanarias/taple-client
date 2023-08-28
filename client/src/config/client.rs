use settings::{ConfigGenerator, SettingSchemaBuilder};
use settings::{ParamType, SettingsMap};
use taple_core::{get_default_settings, DigestDerivator, KeyDerivator, ListenAddr, TapleSettings};

use crate::config::create_path;
use crate::SettingsError;

use super::{extract_boolean, extract_from_map, extract_list, SettingsGenerator};

#[derive(Clone, Debug)]
pub struct ClientSettings {
    pub taple: TapleSettings,
    pub http: bool,
    pub http_addr: String,
    pub http_port: u32,
    pub doc_ui: bool,
    pub database_path: String,
}

impl SettingsGenerator for ClientSettings {
    fn generate(data: &SettingsMap) -> Result<Self, super::error::SettingsError>
    where
        Self: Sized,
    {
        let default_settings = get_default_settings();
        let ports_offset = extract_from_map(&data, "ports-offset", 0u32)?;
        let listen_addr = {
            let mut list: Vec<ListenAddr> = Vec::new();
            let data = {
                let tmp = extract_list(&data, "listen-addr");
                if tmp.is_empty() {
                    default_settings
                        .network
                        .listen_addr
                        .iter()
                        .map(|s| s.to_string().unwrap())
                        .collect()
                } else {
                    tmp
                }
            };
            for addr in data {
                let mut value = ListenAddr::try_from(addr)?;
                value.increment_port(ports_offset);
                list.push(value);
            }
            list
        };
        let mut taple_settings = TapleSettings::generate(data)?;
        taple_settings.network.listen_addr = listen_addr;
        let database_path = create_database_path(&data)?;
        Ok(Self {
            taple: taple_settings,
            http: extract_boolean(data, "http", false)?,
            http_addr: extract_from_map(&data, "addr", "0.0.0.0".into())?,
            http_port: extract_from_map(&data, "port", 3000u32)? + ports_offset,
            doc_ui: extract_from_map(&data, "doc", false)?,
            database_path: database_path,
        })
    }
}

fn create_database_path(data: &SettingsMap) -> Result<String, SettingsError> {
    let path = {
        if let Some(path) = data.get::<String>("db-path") {
            path.clone()
        } else {
            log::warn!("Database path was not defined");
            let path = create_path("db")?;
            log::warn!("Database defaults to {}", path);
            path
        }
    };
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

pub fn client_settings_builder() -> ConfigGenerator {
    let default_settings = taple_core::get_default_settings();
    fn pass_votation_conversion(pass_votation: u8) -> String {
        match pass_votation {
            0 => String::from("never"),
            1 => String::from("always_true"),
            _ => unreachable!(),
        }
    }
    fn key_derivator_conversion(derivator: KeyDerivator) -> String {
        match derivator {
            KeyDerivator::Ed25519 => String::from("ed25519"),
            KeyDerivator::Secp256k1 => String::from("secp256k1"),
        }
    }
    fn digest_derivator_conversion(derivator: DigestDerivator) -> String {
        match derivator {
            DigestDerivator::Blake3_256 => "Blake3_256".into(),
            DigestDerivator::Blake3_512 => "Blake3_512".into(),
            DigestDerivator::SHA2_256 => "SHA2_256".into(),
            DigestDerivator::SHA2_512 => "SHA2_512".into(),
            DigestDerivator::SHA3_256 => "SHA3_256".into(),
            DigestDerivator::SHA3_512 => "SHA3_512".into(),
        }
    }
    ConfigGenerator::new()
        .about("Node for a TAPLE network")
        .program_name(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Open Canarias")
        .usage("taple-client [OPTIONS]")
        .prefix("TAPLE")
        .unwrap()
        .add_toml("settings.toml")
        .group(
            "network",
            Some("network"),
            Some("Network protocol configurations"),
            vec![
                SettingSchemaBuilder::new("listen-addr")
                    .unwrap()
                    .help("Listening address for protocol messages")
                    .param_type(ParamType::Multivalued)
                    .short('a')
                    .build(),
                SettingSchemaBuilder::new("known-node")
                    .unwrap()
                    .param_type(ParamType::Multivalued)
                    .help("Known node at startup")
                    .build(),
                SettingSchemaBuilder::new("external-address")
                    .unwrap()
                    .param_type(ParamType::Multivalued)
                    .help("Known external address at startup")
                    .build(),
            ],
        )
        .unwrap()
        .group(
            "http",
            Option::<String>::None,
            Some("Server HTTP configurations"),
            vec![SettingSchemaBuilder::new("http")
                .unwrap()
                .help("Flag to activate HTTP server")
                .with_default(false.to_string())
                .param_type(ParamType::Flag)
                .build()],
        )
        .unwrap()
        .group(
            "http",
            Some("http"),
            Some("Server HTTP configurations"),
            vec![
                SettingSchemaBuilder::new("port")
                    .unwrap()
                    .help("Port HTTP for the API REST")
                    .build(),
                SettingSchemaBuilder::new("addr")
                    .unwrap()
                    .help("Listening ADDR for the API REST")
                    .build(),
                SettingSchemaBuilder::new("doc")
                    .unwrap()
                    .with_default(false.to_string())
                    .help("Flag to activate OpenAPI documentation endpoint ")
                    .param_type(ParamType::Flag)
                    .build(),
            ],
        )
        .unwrap()
        .group(
            "experimental",
            Option::<String>::None,
            Some("Unstable configurations"),
            vec![
                SettingSchemaBuilder::new("msg-rep-factor")
                    .unwrap()
                    .help("Replication factor to use by the node")
                    .with_default(format!("{}", default_settings.node.replication_factor))
                    .hide(true)
                    .build(),
                SettingSchemaBuilder::new("msg-timeout")
                    .unwrap()
                    .help("Replication factor to use by the node")
                    .with_default(format!("{}", default_settings.node.timeout))
                    .hide(true)
                    .build(),
                SettingSchemaBuilder::new("approval-mode")
                    .unwrap()
                    .help(
                        "To vote to response to all vote request. It requires the dev mode enabled",
                    )
                    .with_default(pass_votation_conversion(default_settings.node.passvotation))
                    .param_type(ParamType::Enum(vec!["never".into(), "always_true".into()]))
                    .hide(true)
                    .build(),
                SettingSchemaBuilder::new("ports-offset")
                    .unwrap()
                    .help("Offset to add to all port used by the node")
                    .hide(true)
                    .build(),
            ],
        )
        .unwrap()
        .group(
            "experimental",
            Some("sc"),
            Some("Unstable configurations"),
            vec![SettingSchemaBuilder::new("build-path")
                .unwrap()
                .help("Path in which contracts are compiled")
                .build()],
        )
        .unwrap()
        .add_setting(
            SettingSchemaBuilder::new("db-path")
                .unwrap()
                .help("Path where to store the database")
                .short('d')
                .build(),
        )
        //
        .add_setting(
            SettingSchemaBuilder::new("id-private-key")
                .unwrap()
                .help("Private Key in hexadecimal to import into the node")
                .short('k')
                .build(),
        )
        .add_setting(
            SettingSchemaBuilder::new("id-key-derivator")
                .unwrap()
                .help("Key derivator used by the private that employs the TAPLE node")
                .param_type(ParamType::Enum(vec!["ed25519".into(), "secp256k1".into()]))
                .with_default(key_derivator_conversion(
                    default_settings.node.key_derivator,
                ))
                .build(),
        )
        .add_setting(
            SettingSchemaBuilder::new("digest-derivator")
                .unwrap()
                .hide(true)
                .help("Digest derivator to use by the TAPLE")
                .with_default(digest_derivator_conversion(
                    default_settings.node.digest_derivator,
                ))
                .param_type(ParamType::Enum(vec![
                    "Blake3_256".into(),
                    "Blake3_512".into(),
                    "SHA2_256".into(),
                    "SHA2_512".into(),
                    "SHA3_256".into(),
                    "SHA3_512".into(),
                ]))
                .build(),
        )
}
