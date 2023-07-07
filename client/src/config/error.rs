use thiserror::Error;
use taple_core::ListenAddrErrors;

#[derive(Error, Debug)]
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
    #[error("Folder creation error {0}")]
    FolderCreationError(#[from] std::io::Error),
    #[error("{0}")]
    ListenAddrError(#[from] ListenAddrErrors)
}
