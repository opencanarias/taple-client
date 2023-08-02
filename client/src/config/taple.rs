use settings::SettingsMap;
use taple_core::{get_default_settings, DigestDerivator, KeyDerivator};
pub use taple_core::{NetworkSettings, NodeSettings, TapleSettings};

use crate::config::create_path;

use super::{
    error::SettingsError, extract_from_map, extract_list, extract_option, SettingsGenerator,
};

impl SettingsGenerator for TapleSettings {
    fn generate(data: &SettingsMap) -> Result<Self, SettingsError> {
        let default_settings = get_default_settings();
        Ok(TapleSettings {
            network: NetworkSettings {
                listen_addr: Vec::new(),
                known_nodes: extract_list(&data, "known-node"),
                external_address: extract_list(&data, "external-addresses"),
            },
            node: NodeSettings {
                key_derivator: extract_key_derivator(
                    &data,
                    "id-key-derivator",
                    default_settings.node.key_derivator,
                )?,
                secret_key: extract_option(&data, "id-private-key")?,
                digest_derivator: extract_digest_derivator(
                    &data,
                    "digest-derivator",
                    default_settings.node.digest_derivator,
                )?,
                replication_factor: extract_from_map(&data, "msg-rep-factor", 0.25f64)?,
                timeout: extract_from_map(&data, "msg-timeout", 3000u32)?,
                passvotation: extract_pass_votation(&data, "approval-mode")?,
                smartcontracts_directory: create_contracts_build_path(&data)?,
            },
        })
    }
}

fn create_contracts_build_path(data: &SettingsMap) -> Result<String, SettingsError> {
    if let Some(path) = data.get::<String>("build-path") {
        Ok(path.clone())
    } else {
        log::warn!("Contract build path was not defined");
        let path = create_path("sc")?;
        log::warn!("Contracts build path defaults to {}", path);
        Ok(path)
    }
}

fn extract_pass_votation<T: Into<String>>(data: &SettingsMap, key: T) -> Result<u8, SettingsError> {
    let key: String = key.into();
    let Some(value) = data.get::<String>(&key) else {
        return Ok(0u8);
    };
    match value.as_str() {
        "never" => Ok(0u8),
        "always_true" => Ok(1u8),
        _ => Err(SettingsError::InvalidPassVotation),
    }
}

fn extract_key_derivator<T: Into<String>>(
    data: &SettingsMap,
    key: T,
    default: KeyDerivator,
) -> Result<KeyDerivator, SettingsError> {
    let key: String = key.into();
    let Some(value) = data.get::<String>(&key) else {
        return Ok(default);
    };
    match value.as_str() {
        "ed25519" => Ok(KeyDerivator::Ed25519),
        "secp256k1" => Ok(KeyDerivator::Secp256k1),
        _ => Err(SettingsError::InvalidKeyDerivator),
    }
}

fn extract_digest_derivator<T: Into<String>>(
    data: &SettingsMap,
    key: T,
    default: DigestDerivator,
) -> Result<DigestDerivator, SettingsError> {
    let key: String = key.into();
    let Some(value) = data.get::<String>(&key) else {
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
