use thiserror::Error;
use warp::reject;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Request Error {0}")]
    RequestError(String),
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Execution Error")]
    ExecutionError,
    #[error("Invalid Parameters")]
    InvalidParameters,
    #[error("Not found")]
    NotFound,
    #[error("Not enough permissions")]
    NotEnoughPermissions,
    #[error("Unauthorized. Invalud API KEY")]
    Unauthorized,
}

impl reject::Reject for Error {}
