use settings::ParamType;
use settings::{ConfigGenerator, SettingSchemaBuilder};
use std::collections::HashMap;
use taple_core::TapleSettings;

use super::{extract_from_map, SettingsGenerator};

#[derive(Clone, Debug)]
pub struct ClientSettings {
    pub taple: TapleSettings,
    pub http_addr: String,
    pub http_port: u32,
    pub swagger_ui: bool,
}

impl SettingsGenerator for ClientSettings {
    fn generate(data: &HashMap<String, String>) -> Result<Self, super::error::SettingsError>
    where
        Self: Sized,
    {
        let taple_settings = TapleSettings::generate(data)?;
        Ok(Self {
            taple: taple_settings,
            http_addr: extract_from_map(&data, "httpaddr", "0.0.0.0".into())?,
            http_port: extract_from_map(&data, "httpport", 3000u32)?,
            swagger_ui: extract_from_map(&data, "swaggerui", false)?,
        })
    }
}

pub fn client_settings_builder() -> ConfigGenerator {
    ConfigGenerator::new()
        .about("Node for a TAPLE network")
        .program_name(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Open Canarias")
        .usage("taple-client [OPTIONS]")
        .prefix("TAPLE").unwrap()
        .add_toml("settings.toml")
        .group(
            "network",
            Some("network"),
            Some("Network protocol configurations"),
            vec![
                SettingSchemaBuilder::new("p2pport")
                    .unwrap()
                    .help("Port for the node to listen for protocol messages")
                    .short('p')
                    .build(),
                SettingSchemaBuilder::new("addr")
                    .unwrap()
                    .help("Listening address for protocol messages")
                    .short('a')
                    .build(),
                SettingSchemaBuilder::new("knownnodes")
                    .unwrap()
                    .help("List of access points to use by the node. Each element is separated by ';'")
                    .build(),
        ]).unwrap()
        .group(
            "node",
            Some("node"),
            Some("Cryptographic and general configuration"),
            vec![
                SettingSchemaBuilder::new("seed")
                    .unwrap()
                    .help("Seed to use to generate the MC")
                    .short('s')
                    .build(),
                SettingSchemaBuilder::new("secretkey")
                    .unwrap()
                    .help("Secret Key in hexadecimal to import into the node")
                    .short('k')
                    .build(),
                SettingSchemaBuilder::new("keyderivator")
                    .unwrap()
                    .help("Key derivator to use by the TAPLE")
                    .param_type(ParamType::Enum(vec!["ed25519".into(), "secp256k1".into()]))
                    .build(),
                SettingSchemaBuilder::new("digestderivator")
                    .unwrap()
                    .help("Digest derivator to use by the TAPLE")
                    .param_type(ParamType::Enum(vec![
                        "Blake3_256".into(),
                        "Blake3_512".into(),
                        "SHA2_256".into(),
                        "SHA2_512".into(),
                        "SHA3_256".into(),
                        "SHA3_512".into(),
                    ]))
                    .build(),
                SettingSchemaBuilder::new("factor")
                    .unwrap()
                    .help("Replication factor to use by the node")
                    .short('f')
                    .build(),
                SettingSchemaBuilder::new("timeout")
                    .unwrap()
                    .help("Replication factor to use by the node")
                    .short('t')
                    .build(),
                SettingSchemaBuilder::new("devmode")
                    .unwrap()
                    .help("Flag to activate the developer mode")
                    .short('m')
                    .param_type(ParamType::Flag)
                    .build(),
                SettingSchemaBuilder::new("passvotation")
                    .unwrap()
                    .help("To vote to response to all vote request. It requires the dev mode enabled")
                    .short('v')
                    .param_type(ParamType::Enum(vec![
                        "dissabled".into(),
                        "always_yes".into(),
                        "always_no".into(),
                    ]))
                    .build(),
        ]).unwrap()
        .group(
            "database",
            Some("database"), 
            Some("Database configurations"),
            vec![
                SettingSchemaBuilder::new("path")
                    .unwrap()
                    .help("Path where to store the database")
                    .short('d')
                    .build(),
        ]).unwrap()
        .add_setting(
            SettingSchemaBuilder::new("httpport")
                .unwrap()
                .help("Port HTTP for the API REST")
                .build(),
        )
        .add_setting(
            SettingSchemaBuilder::new("httpaddr")
                .unwrap()
                .help("Listening ADDR for the API REST")
                .build(),
        )
        .add_setting(
            SettingSchemaBuilder::new("swaggerui")
                .unwrap()
                .help("Flag to activate swagger-ui")
                .param_type(ParamType::Flag)
                .build(),
        )
        .add_setting(
            SettingSchemaBuilder::new("contractsdkpath")
                .unwrap()
                .help("Path to the smart contracts SDK")
                .build(),
        )
}
