use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum SettingsError {
    #[error("Parameter {0} not found")]
    ParameterNotFound(String),
    #[error("Invalid type parameter for {0}")]
    InvalidTypeParamer(String),
    #[error("Invalid KeyDerivator")]
    InvalidKeyDerivator,
    #[error("Invalid DigestDerivator")]
    InvalidDigestDerivator,
    #[error("Invalid PassVotation")]
    InvalidPassVotation,
}
