mod client;
mod error;
mod taple;

pub use self::client::{client_settings_builder, ClientSettings};
use easy_settings::SettingsMap;
pub use error::SettingsError;
pub use taple::Settings;

pub trait SettingsGenerator {
    fn generate(data: &SettingsMap) -> Result<Self, SettingsError>
    where
        Self: Sized;
}

pub fn create_path(name: &str) -> Result<String, SettingsError> {
    let path = if let Some(home_path) = home::home_dir() {
        home_path
    } else {
        std::env::temp_dir()
    };
    let path = format!("{}/.taple/{}", path.display(), name);
    Ok(path)
}

pub fn extract_list<T: Into<String>>(data: &SettingsMap, key: T) -> Vec<String> {
    let key: String = key.into();
    let Some(list) = data.get::<Vec<String>>(&key) else {
        return Vec::new();
    };
    list.to_owned()
}

pub fn extract_option<T: Into<String>, S: std::str::FromStr>(
    data: &SettingsMap,
    key: T,
) -> Result<Option<S>, SettingsError> {
    let key: String = key.into();
    let Some(value) = data.get::<String>(&key) else {
      return Ok(None);
    };
    Ok(Some(value.parse::<S>().map_err(|_| {
        SettingsError::InvalidTypeParamer(key.clone())
    })?))
}

pub fn extract_from_map<T: Into<String>, S: std::str::FromStr>(
    data: &SettingsMap,
    key: T,
    default: S,
) -> Result<S, SettingsError> {
    let key: String = key.into();
    let Some(value) = data.get::<String>(&key) else {
        return Ok(default);
    };
    value
        .parse::<S>()
        .map_err(|_| SettingsError::InvalidTypeParamer(key.clone()))
}

pub fn extract_boolean<T: Into<String>>(
    data: &SettingsMap,
    key: T,
    default: bool,
) -> Result<bool, SettingsError> {
    let key: String = key.into();
    if let Some(value) = data.get::<String>(&key) {
        if value == "true" {
            return Ok(true);
        } else if value == "false" {
            return Ok(false);
        }
    };
    let Some(value) = data.get::<bool>(&key) else {
        return Ok(default);
    };
    Ok(*value)
}
