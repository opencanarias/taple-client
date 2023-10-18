pub mod bodys;
pub mod error;
pub mod handlers;
pub mod querys;
pub mod responses;

use super::api::error::Error;
use super::api::handlers::*;
use super::api::querys::*;
use super::api::responses::ErrorResponse;
use serde::de::DeserializeOwned;
use taple_core::crypto::KeyPair;
use taple_core::DigestDerivator;
use taple_core::{Api, KeyDerivator};
use warp::body::BodyDeserializeError;
use warp::{http::Response, hyper::StatusCode, Filter, Rejection, Reply};

const API_BASE_PATH: &str = "api";

pub fn routes(
    taple_api: Api,
    keys: KeyPair,
    derivator: KeyDerivator,
    digest_derivator: DigestDerivator,
    payload_size: u64
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let root = warp::path(API_BASE_PATH);

    root.and(
        get_subject(taple_api.clone())
            .or(get_all_subjects(taple_api.clone()))
            .or(get_subject(taple_api.clone()))
            .or(post_event_request(
                taple_api.clone(),
                keys,
                derivator,
                digest_derivator,
                payload_size,
            ))
            .or(get_events_of_subject(taple_api.clone()))
            .or(get_event(taple_api.clone()))
            .or(patch_approval(taple_api.clone(), payload_size))
            .or(post_preauthorized_subjects(taple_api.clone(), payload_size))
            .or(get_preauthorized_subjects(taple_api.clone()))
            .or(get_events_of_subject(taple_api.clone()))
            .or(get_validation_proof(taple_api.clone()))
            .or(post_generate_keys(taple_api.clone()))
            .or(get_event_request(taple_api.clone()))
            .or(get_approval(taple_api.clone()))
            .or(get_pending_approvals(taple_api.clone()))
            .or(get_event_request_state(taple_api))
            .recover(handle_rejection),
    )
}

pub fn get_approval(
    taple_api: Api,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("approval-requests" / String)
        .and(warp::get())
        .and(with_taple_api(taple_api))
        .and_then(get_approval_handler)
}

pub fn get_pending_approvals(
    taple_api: Api,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("approval-requests")
        .and(warp::get())
        .and(with_taple_api(taple_api))
        .and(warp::query::<GetApprovalsQuery>())
        .and_then(get_approvals_handler)
}

pub fn get_event_request(
    taple_api: Api,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("event-requests" / String)
        .and(warp::get())
        .and(with_taple_api(taple_api))
        .and_then(get_taple_request_handler)
}

pub fn get_event_request_state(
    taple_api: Api,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("event-requests" / String / "state")
        .and(warp::get())
        .and(with_taple_api(taple_api))
        .and_then(get_taple_request_state_handler)
}

pub fn get_subject(taple_api: Api) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("subjects" / String)
        .and(warp::get())
        .and(with_taple_api(taple_api))
        .and_then(get_subject_handler)
}

pub fn get_all_subjects(
    taple_api: Api,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("subjects")
        .and(warp::get())
        .and(with_taple_api(taple_api))
        .and(warp::query::<GetAllSubjectsQuery>())
        .and_then(get_subjects_handler)
}

pub fn post_event_request(
    taple_api: Api,
    keys: KeyPair,
    derivator: KeyDerivator,
    digest_derivator: DigestDerivator,
    payload_size: u64
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("event-requests")
        .and(warp::post())
        .and(with_taple_api(taple_api))
        .and(with_keys(keys))
        .and(with_derivator(derivator))
        .and(with_digest_derivator(digest_derivator))
        .and(with_body(payload_size))
        .and_then(post_event_request_handler)
}

pub fn patch_approval(
    taple_api: Api,
    payload_size: u64
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("approval-requests" / String)
        .and(warp::patch())
        .and(with_taple_api(taple_api))
        .and(with_body(payload_size))
        .and_then(patch_approval_handler)
}

pub fn post_generate_keys(
    taple_api: Api,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("keys")
        .and(warp::post())
        .and(with_taple_api(taple_api))
        .and(warp::query::<AddKeysQuery>())
        .and_then(post_generate_keys_handler)
}

pub fn post_preauthorized_subjects(
    taple_api: Api,
    payload_size: u64
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("allowed-subjects" / String)
        .and(warp::put())
        .and(with_taple_api(taple_api))
        .and(with_body(payload_size))
        .and_then(put_allowed_subjects_handler)
}

pub fn get_preauthorized_subjects(
    taple_api: Api,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("allowed-subjects")
        .and(warp::get())
        .and(with_taple_api(taple_api))
        .and(warp::query::<GetWithPaginationString>())
        .and_then(get_allowed_subjects_handler)
}

pub fn get_events_of_subject(
    taple_api: Api,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("subjects" / String / "events")
        .and(warp::get())
        .and(with_taple_api(taple_api))
        .and(warp::query::<GetWithPagination>())
        .and_then(get_events_of_subject_handler)
}

pub fn get_event(taple_api: Api) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("subjects" / String / "events" / u64)
        .and(warp::get())
        .and(with_taple_api(taple_api))
        .and_then(get_event_handler)
}

pub fn get_validation_proof(
    taple_api: Api,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("subjects" / String / "validation")
        .and(warp::get())
        .and(with_taple_api(taple_api))
        .and_then(get_validation_proof_handle)
}

pub fn with_taple_api(
    taple_api: Api,
) -> impl Filter<Extract = (Api,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || taple_api.clone())
}

pub fn with_keys(
    keys: KeyPair,
) -> impl Filter<Extract = (KeyPair,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || keys.clone())
}

pub fn with_derivator(
    derivator: KeyDerivator,
) -> impl Filter<Extract = (KeyDerivator,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || derivator)
}

pub fn with_digest_derivator(
    derivator: DigestDerivator,
) -> impl Filter<Extract = (DigestDerivator,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || derivator)
}

pub fn with_body<T: DeserializeOwned + Send>(
    size: u64
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(size).and(warp::body::json())
}

// TODO: refactor errors
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    let (msg, status_code) = if let Some(ref err) = err.find::<Error>() {
        match err {
            Error::InternalServerError { error } => {
                (error.to_owned(), StatusCode::INTERNAL_SERVER_ERROR)
            }
            Error::ExecutionError { source } => (source.to_string(), StatusCode::CONFLICT),
            Error::InvalidParameters { error } => (error.to_string(), StatusCode::BAD_REQUEST),
            Error::NotEnoughPermissions { error } => (error.to_string(), StatusCode::FORBIDDEN),
            Error::NotFound { error } => (error.to_string(), StatusCode::NOT_FOUND),
            Error::Unauthorized { error } => (error.to_string(), StatusCode::UNAUTHORIZED),
            Error::BadRequest { error } => (error.to_string(), StatusCode::BAD_REQUEST),
            Error::Conflict { error } => (error.to_string(), StatusCode::CONFLICT),
        }
    } else if err.is_not_found() {
        ("Not Found".to_owned(), StatusCode::NOT_FOUND)
    } else if let Some(ref err) = err.find::<BodyDeserializeError>() {
        (err.to_string(), StatusCode::BAD_REQUEST)
    } else if let Some(ref err) = err.find::<warp::reject::MethodNotAllowed>() {
        (err.to_string(), StatusCode::METHOD_NOT_ALLOWED)
    } else if let Some(ref err) = err.find::<warp::reject::InvalidHeader>() {
        (err.to_string(), StatusCode::BAD_REQUEST)
    } else if let Some(ref err) = err.find::<warp::reject::MissingCookie>() {
        (err.to_string(), StatusCode::BAD_REQUEST)
    } else if let Some(ref err) = err.find::<warp::reject::PayloadTooLarge>() {
        (err.to_string(), StatusCode::BAD_REQUEST)
    } else if let Some(ref err) = err.find::<warp::reject::MissingHeader>() {
        (err.to_string(), StatusCode::BAD_REQUEST)
    } else if let Some(ref err) = err.find::<warp::reject::InvalidQuery>() {
        (err.to_string(), StatusCode::BAD_REQUEST)
    } else if let Some(ref err) = err.find::<warp::reject::UnsupportedMediaType>() {
        (err.to_string(), StatusCode::BAD_REQUEST)
    } else if let Some(ref err) = err.find::<warp::reject::LengthRequired>() {
        (err.to_string(), StatusCode::BAD_REQUEST)
    } else {
        return Err(err);
    };
    let error = ErrorResponse {
        code: status_code.as_u16(),
        error: msg,
    };
    let json_response = warp::reply::json(&error);
    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .status(status_code)
        .body(json_response.into_response().into_body()))
}
