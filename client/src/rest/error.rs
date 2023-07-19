use taple_core::ApiError;
use thiserror::Error;
use warp::reject;

#[allow(dead_code)]
#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("{}", source)]
    ExecutionError {
        #[from]
        source: ApiError
    },
    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),
    #[error("Not found {0}")]
    NotFound(String),
    #[error("Not enough permissions")]
    NotEnoughPermissions,
    #[error("Unauthorized. Invalid API KEY")]
    Unauthorized,
}

impl reject::Reject for Error {}
