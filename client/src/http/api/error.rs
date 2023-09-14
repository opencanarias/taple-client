use taple_core::ApiError;
use thiserror::Error;
use warp::reject;

#[allow(dead_code)]
#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Bad Request: {}", error)]
    BadRequest { error: String },
    #[error("Internal Server Error: {}", error)]
    InternalServerError { error: String },
    #[error("{}", source)]
    ExecutionError {
        #[from]
        source: ApiError,
    },
    #[error("Invalid parameters: {}", error)]
    InvalidParameters { error: String },
    #[error("Not found {}", error)]
    NotFound { error: String },
    #[error("Not enough permissions")]
    NotEnoughPermissions { error: String },
    #[error("Unauthorized. Invalid API KEY")]
    Unauthorized { error: String },
    #[error("Conflict: {}", error)]
    Conflict { error: String },
}

impl reject::Reject for Error {}
