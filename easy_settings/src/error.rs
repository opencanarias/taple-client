use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Empty String as ID or ENV")]
    EmptyString,
    #[error("The string specified is not a valid name for an env: {0}")]
    InvalidStringForEnv(String),
}
