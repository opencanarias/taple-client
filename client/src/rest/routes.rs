use crate::rest::querys::AddKeysQuery;
use crate::rest::querys::GetWithPaginationString;

use super::handlers::{
    get_allowed_subjects_handler, get_approval_handler, get_approvals_handler, get_event_handler,
    get_events_of_subject_handler, get_subject_handler, get_subjects_handler,
    get_taple_request_handler, get_taple_request_state_handler, get_validation_proof_handle,
    patch_approval_handler, post_event_request_handler, post_generate_keys_handler,
    put_allowed_subjects_handler,
};
use super::querys::GetApprovalsQuery;
use super::responses::ErrorResponse;
use super::{
    error::Error,
    querys::{GetAllSubjectsQuery, GetWithPagination},
};
use serde::de::DeserializeOwned;
use taple_core::crypto::KeyPair;
use taple_core::{KeyDerivator, NodeAPI};
use warp::body::BodyDeserializeError;
use warp::{hyper::StatusCode, reply::Response, Filter, Rejection, Reply};

pub fn routes(
    sender: NodeAPI,
    keys: KeyPair,
    derivator: KeyDerivator,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_subject(sender.clone())
        .or(get_all_subjects(sender.clone()))
        .or(get_subject(sender.clone()))
        .or(post_event_request(
            sender.clone(),
            keys.clone(),
            derivator.clone(),
        ))
        .or(get_events_of_subject(sender.clone()))
        .or(get_event(sender.clone()))
        .or(patch_approval(sender.clone()))
        .or(post_preauthorized_subjects(sender.clone()))
        .or(get_preauthorized_subjects(sender.clone()))
        .or(get_events_of_subject(sender.clone()))
        .or(get_validation_proof(sender.clone()))
        .or(post_generate_keys(sender.clone()))
        .or(get_event_request(sender.clone()))
        .or(get_approval(sender.clone()))
        .or(get_pending_approvals(sender.clone()))
        .or(get_event_request_state(sender.clone()))
        .recover(handle_rejection)
}

pub fn get_approval(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "approval-requests" / String)
        .and(warp::get())
        .and(with_sender(sender))
        .and_then(get_approval_handler)
}

pub fn get_pending_approvals(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "approval-requests")
        .and(warp::get())
        .and(with_sender(sender))
        .and(warp::query::<GetApprovalsQuery>())
        .and_then(get_approvals_handler)
}

pub fn get_event_request(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "event-requests" / String)
        .and(warp::get())
        .and(with_sender(sender))
        .and_then(get_taple_request_handler)
}

pub fn get_event_request_state(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "event-requests" / String / "state")
        .and(warp::get())
        .and(with_sender(sender))
        .and_then(get_taple_request_state_handler)
}

pub fn get_subject(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "subjects" / String)
        .and(warp::get())
        .and(with_sender(sender))
        .and_then(get_subject_handler)
}

pub fn get_all_subjects(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "subjects")
        .and(warp::get())
        .and(with_sender(sender))
        .and(warp::query::<GetAllSubjectsQuery>())
        .and_then(get_subjects_handler)
}

pub fn post_event_request(
    sender: NodeAPI,
    keys: KeyPair,
    derivator: KeyDerivator,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "event-requests")
        .and(warp::post())
        .and(with_sender(sender))
        .and(with_keys(keys))
        .and(with_derivator(derivator))
        .and(with_body())
        .and_then(post_event_request_handler)
}

pub fn patch_approval(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "approval-requests" / String)
        .and(warp::patch())
        //.and(warp::header("X-API-KEY"))
        .and(with_sender(sender))
        .and(with_body())
        .and_then(patch_approval_handler)
}

pub fn post_generate_keys(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "keys")
        .and(warp::post())
        .and(with_sender(sender))
        .and(warp::query::<AddKeysQuery>())
        .and_then(post_generate_keys_handler)
}

pub fn post_preauthorized_subjects(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "allowed-subjects" / String)
        .and(warp::put())
        .and(with_sender(sender))
        .and(with_body())
        .and_then(put_allowed_subjects_handler)
}

pub fn get_preauthorized_subjects(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "allowed-subjects")
        .and(warp::get())
        .and(with_sender(sender))
        .and(warp::query::<GetWithPaginationString>())
        .and_then(get_allowed_subjects_handler)
}

pub fn get_events_of_subject(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "subjects" / String / "events")
        .and(warp::get())
        .and(with_sender(sender))
        .and(warp::query::<GetWithPagination>())
        .and_then(get_events_of_subject_handler)
}

pub fn get_event(sender: NodeAPI) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "subjects" / String / "events" / u64)
        .and(warp::get())
        .and(with_sender(sender))
        .and_then(get_event_handler)
}

pub fn get_validation_proof(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "subjects" / String / "validation")
        .and(warp::get())
        .and(with_sender(sender))
        .and_then(get_validation_proof_handle)
}

pub fn with_sender(
    sender: NodeAPI,
) -> impl Filter<Extract = (NodeAPI,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || sender.clone())
}

pub fn with_keys(
    keys: KeyPair,
) -> impl Filter<Extract = (KeyPair,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || keys.clone())
}

pub fn with_derivator(
    derivator: KeyDerivator,
) -> impl Filter<Extract = (KeyDerivator,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || derivator.clone())
}

pub fn with_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    println!("HANDLE REJECTION: {:?}", err);
    if let Some(ref err) = err.find::<Error>() {
        match err {
            Error::InternalServerError { error } => {
                let error = ErrorResponse {
                    code: 500,
                    error: error.to_owned(),
                };
                let json_response = warp::reply::json(&error);
                let mut response = Response::new(json_response.into_response().into_body());
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                return Ok(response);
            }
            Error::ExecutionError { source } => {
                let error = ErrorResponse {
                    code: 500,
                    error: source.to_string(),
                };
                let json_response = warp::reply::json(&error);
                let mut response = Response::new(json_response.into_response().into_body());
                *response.status_mut() = StatusCode::CONFLICT;
                return Ok(response);
            }
            Error::InvalidParameters { error } => {
                let error = ErrorResponse {
                    code: 400,
                    error: error.to_owned(),
                };
                let json_response = warp::reply::json(&error);
                let mut response = Response::new(json_response.into_response().into_body());
                *response.status_mut() = StatusCode::BAD_REQUEST;
                return Ok(response);
            }
            Error::NotEnoughPermissions { error } => {
                let error = ErrorResponse {
                    code: 403,
                    error: error.to_owned(),
                };
                let json_response = warp::reply::json(&error);
                let mut response = Response::new(json_response.into_response().into_body());
                *response.status_mut() = StatusCode::FORBIDDEN;
                return Ok(response);
            }
            Error::NotFound { error } => {
                let error = ErrorResponse {
                    code: 404,
                    error: error.to_owned(),
                };
                let json_response = warp::reply::json(&error);
                let mut response = Response::new(json_response.into_response().into_body());
                *response.status_mut() = StatusCode::NOT_FOUND;
                return Ok(response);
            }
            Error::Unauthorized { error } => {
                let error = ErrorResponse {
                    code: 401,
                    error: error.to_owned(),
                };
                let json_response = warp::reply::json(&error);
                let mut response = Response::new(json_response.into_response().into_body());
                *response.status_mut() = StatusCode::UNAUTHORIZED;
                return Ok(response);
            }
            Error::BadRequest { error } => {
                let error = ErrorResponse {
                    code: 400,
                    error: error.to_owned(),
                };
                let json_response = warp::reply::json(&error);
                let mut response = Response::new(json_response.into_response().into_body());
                *response.status_mut() = StatusCode::BAD_REQUEST;
                return Ok(response);
            }
            Error::Conflict { error } => {
                let error = ErrorResponse {
                    code: 409,
                    error: error.to_owned(),
                };
                let json_response = warp::reply::json(&error);
                let mut response = Response::new(json_response.into_response().into_body());
                *response.status_mut() = StatusCode::CONFLICT;
                return Ok(response);
            }
        }
    } else if err.is_not_found() {
        let error = ErrorResponse {
            code: 404,
            error: "Not Found".to_owned(),
        };
        let json_response = warp::reply::json(&error);
        let mut response = Response::new(json_response.into_response().into_body());
        *response.status_mut() = StatusCode::NOT_FOUND;
        return Ok(response);
    } else if let Some(ref err) = err.find::<BodyDeserializeError>() {
        let error = ErrorResponse {
            code: 400,
            error: err.to_string(),
        };
        let json_response = warp::reply::json(&error);
        let mut response = Response::new(json_response.into_response().into_body());
        *response.status_mut() = StatusCode::BAD_REQUEST;
        return Ok(response);
    } else if let Some(ref err) = err.find::<warp::reject::MethodNotAllowed>() {
        let error = ErrorResponse {
            code: 405,
            error: err.to_string(),
        };
        let json_response = warp::reply::json(&error);
        let mut response = Response::new(json_response.into_response().into_body());
        *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
        return Ok(response);
    } else if let Some(ref err) = err.find::<warp::reject::InvalidHeader>() {
        let error = ErrorResponse {
            code: 400,
            error: err.to_string(),
        };
        let json_response = warp::reply::json(&error);
        let mut response = Response::new(json_response.into_response().into_body());
        *response.status_mut() = StatusCode::BAD_REQUEST;
        return Ok(response);
    } else if let Some(ref err) = err.find::<warp::reject::MissingCookie>() {
        let error = ErrorResponse {
            code: 400,
            error: err.to_string(),
        };
        let json_response = warp::reply::json(&error);
        let mut response = Response::new(json_response.into_response().into_body());
        *response.status_mut() = StatusCode::BAD_REQUEST;
        return Ok(response);
    } else if let Some(ref err) = err.find::<warp::reject::PayloadTooLarge>() {
        let error = ErrorResponse {
            code: 400,
            error: err.to_string(),
        };
        let json_response = warp::reply::json(&error);
        let mut response = Response::new(json_response.into_response().into_body());
        *response.status_mut() = StatusCode::BAD_REQUEST;
        return Ok(response);
    } else if let Some(ref err) = err.find::<warp::reject::MissingHeader>() {
        let error = ErrorResponse {
            code: 400,
            error: err.to_string(),
        };
        let json_response = warp::reply::json(&error);
        let mut response = Response::new(json_response.into_response().into_body());
        *response.status_mut() = StatusCode::BAD_REQUEST;
        return Ok(response);
    } else if let Some(ref err) = err.find::<warp::reject::InvalidQuery>() {
        let error = ErrorResponse {
            code: 400,
            error: err.to_string(),
        };
        let json_response = warp::reply::json(&error);
        let mut response = Response::new(json_response.into_response().into_body());
        *response.status_mut() = StatusCode::BAD_REQUEST;
        return Ok(response);
    } else if let Some(ref err) = err.find::<warp::reject::UnsupportedMediaType>() {
        let error = ErrorResponse {
            code: 400,
            error: err.to_string(),
        };
        let json_response = warp::reply::json(&error);
        let mut response = Response::new(json_response.into_response().into_body());
        *response.status_mut() = StatusCode::BAD_REQUEST;
        return Ok(response);
    } else if let Some(ref err) = err.find::<warp::reject::LengthRequired>() {
        let error = ErrorResponse {
            code: 400,
            error: err.to_string(),
        };
        let json_response = warp::reply::json(&error);
        let mut response = Response::new(json_response.into_response().into_body());
        *response.status_mut() = StatusCode::BAD_REQUEST;
        return Ok(response);
    } else {
        Err(err)
    }
}
