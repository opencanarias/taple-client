use std::collections::HashMap;
pub use taple_core::{DatabaseSettings, NetworkSettings, NodeSettings, TapleSettings};
use taple_core::{DigestDerivator, KeyDerivator};

use super::{error::SettingsError, extract_from_map, extract_option, SettingsGenerator};

impl SettingsGenerator for TapleSettings {
    fn generate(data: &HashMap<String, String>) -> Result<Self, SettingsError> {
        let database_path = if let Some(path) = data.get("path") {
            path.clone()
        } else {
            log::warn!("Database path was not defined");
            let path = if let Some(home_path) = home::home_dir() {
                home_path
            } else {
                std::env::temp_dir()
            };
            let path = format!("{}/.taple/db", path.display());
            log::warn!("Database defaults to {}", path);
            path
        };
        Ok(TapleSettings {
            network: NetworkSettings {
                p2p_port: extract_from_map(&data, "p2pport", 0)?,
                addr: extract_from_map(&data, "addr", "/ip4/0.0.0.0/tcp".into())?,
                known_nodes: extract_known_nodes(&data, "knownnodes"),
                external_address: extract_known_nodes(&data, "externaladdress"),
            },
            node: NodeSettings {
                key_derivator: extract_key_derivator(&data, "keyderivator", KeyDerivator::Ed25519)?,
                secret_key: extract_option(&data, "secretkey")?,
                seed: extract_option(&data, "seed")?,
                digest_derivator: extract_digest_derivator(
                    &data,
                    "digestderivator",
                    DigestDerivator::Blake3_256,
                )?,
                replication_factor: extract_from_map(&data, "factor", 0.25f64)?,
                timeout: extract_from_map(&data, "timeout", 3000u32)?,
                passvotation: extract_pass_votation(&data, "passvotation")?,
                dev_mode: extract_from_map(&data, "devmode", false)?,
                req_res: false,
                smartcontracts_directory: String::from("../contracts"), // TODO: CAMBIAR EN UN FUTURO
            },
            database: DatabaseSettings {
                path: database_path,
            },
        })
    }
}

fn extract_known_nodes<T: Into<String>>(data: &HashMap<String, String>, key: T) -> Vec<String> {
    let key: String = key.into();
    let Some(nodes) = data.get(&key) else {
        return Vec::new();
    };
    nodes.split(';').map(|f| f.to_string()).collect()
}

fn extract_pass_votation<T: Into<String>>(
    data: &HashMap<String, String>,
    key: T,
) -> Result<u8, SettingsError> {
    let key: String = key.into();
    let Some(value) = data.get(&key) else {
    return Ok(0u8);
  };
    match value.as_str() {
        "dissabled" => Ok(0u8),
        "always_yes" => Ok(1u8),
        "always_no" => Ok(2u8),
        _ => Err(SettingsError::InvalidPassVotation),
    }
}

fn extract_key_derivator<T: Into<String>>(
    data: &HashMap<String, String>,
    key: T,
    default: KeyDerivator,
) -> Result<KeyDerivator, SettingsError> {
    let key: String = key.into();
    let Some(value) = data.get(&key) else {
    return Ok(default);
  };
    match value.as_str() {
        "ed25519" => Ok(KeyDerivator::Ed25519),
        "secp256k1" => Ok(KeyDerivator::Secp256k1),
        _ => Err(SettingsError::InvalidKeyDerivator),
    }
}

fn extract_digest_derivator<T: Into<String>>(
    data: &HashMap<String, String>,
    key: T,
    default: DigestDerivator,
) -> Result<DigestDerivator, SettingsError> {
    let key: String = key.into();
    let Some(value) = data.get(&key) else {
        return Ok(default);
    };
    match value.as_str() {
        "Blake3_256" => Ok(DigestDerivator::Blake3_256),
        "Blake3_512" => Ok(DigestDerivator::Blake3_512),
        "SHA2_256" => Ok(DigestDerivator::SHA2_256),
        "SHA2_512" => Ok(DigestDerivator::SHA2_512),
        "SHA3_256" => Ok(DigestDerivator::SHA3_256),
        "SHA3_512" => Ok(DigestDerivator::SHA3_512),
        _ => Err(SettingsError::InvalidDigestDerivator),
    }
}
