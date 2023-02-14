use std::collections::HashMap;

mod client;
mod error;
mod taple;

pub use self::client::{client_settings_builder, ClientSettings};
pub use error::SettingsError;
pub use taple::TapleSettings;

pub trait SettingsGenerator {
    fn generate(data: &HashMap<String, String>) -> Result<Self, SettingsError>
    where
        Self: Sized;
}

pub fn extract_option<T: Into<String>, S: std::str::FromStr>(
    data: &HashMap<String, String>,
    key: T,
) -> Result<Option<S>, SettingsError> {
    let key: String = key.into();
    let Some(value) = data.get(&key) else {
      return Ok(None);
    };
    Ok(Some(value.parse::<S>().map_err(|_| {
        SettingsError::InvalidTypeParamer(key.clone())
    })?))
}

pub fn extract_from_map<T: Into<String>, S: std::str::FromStr>(
    data: &HashMap<String, String>,
    key: T,
    default: S,
) -> Result<S, SettingsError> {
    let key: String = key.into();
    let Some(value) = data.get(&key) else {
    return Ok(default);
  };
    value
        .parse::<S>()
        .map_err(|_| SettingsError::InvalidTypeParamer(key.clone()))
}
