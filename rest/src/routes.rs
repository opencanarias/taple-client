use crate::handlers::{
    get_single_request_handler, post_event_request_handler,
};

use super::handlers::{
    get_all_governances_handler, get_all_subjects_handler, get_event_handler,
    get_event_properties_handler, get_events_of_subject_handler, get_governance_handler,
    get_pending_requests_handler, get_subject_handler, put_approval_handler,
};
use super::{
    error::Error,
    querys::{GetAllSubjectsQuery, GetEventsQuery},
};
use taple_core::NodeAPI;
use serde::de::DeserializeOwned;
use warp::{hyper::StatusCode, reply::Response, Filter, Rejection, Reply};

pub fn routes(
    sender: NodeAPI,
    api_key: Option<String>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_subject(sender.clone(), api_key.clone())
        .or(get_all_subjects(sender.clone(), api_key.clone()))
        .or(get_all_governances(sender.clone(), api_key.clone()))
        .or(get_subject(sender.clone(), api_key.clone()))
        .or(post_event_request(sender.clone(), api_key.clone()))
        .or(get_governance(sender.clone(), api_key.clone()))
        .or(get_events_of_subject(sender.clone(), api_key.clone()))
        .or(get_event(sender.clone(), api_key.clone()))
        .or(get_event_properties(sender.clone(), api_key.clone()))
        .or(put_approval(sender.clone(), api_key.clone()))
        .or(get_single_request(sender.clone(), api_key.clone()))
        .or(get_pending_requests(sender.clone(), api_key.clone()))
}

fn get_single_request(
    sender: NodeAPI,
    api_key: Option<String>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "approvals" / String)
        .and(warp::get())
        .and(with_sender(sender))
        .and(api_key_validation(api_key))
        .and_then(get_single_request_handler)
        .recover(handle_rejection)
}

fn get_pending_requests(
    sender: NodeAPI,
    api_key: Option<String>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "approvals")
        .and(warp::get())
        .and(with_sender(sender))
        .and(api_key_validation(api_key))
        .and_then(get_pending_requests_handler)
        .recover(handle_rejection)
}

fn get_subject(
    sender: NodeAPI,
    api_key: Option<String>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "subjects" / String)
        .and(warp::get())
        .and(with_sender(sender))
        .and(api_key_validation(api_key))
        .and_then(get_subject_handler)
        .recover(handle_rejection)
}

fn get_all_subjects(
    sender: NodeAPI,
    api_key: Option<String>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "subjects")
        .and(warp::get())
        .and(with_sender(sender))
        .and(api_key_validation(api_key))
        .and(warp::query::<GetAllSubjectsQuery>())
        .and_then(get_all_subjects_handler)
        .recover(handle_rejection)
}

fn get_governance(
    sender: NodeAPI,
    api_key: Option<String>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "governances" / String)
        .and(warp::get())
        .and(with_sender(sender))
        .and(api_key_validation(api_key))
        .and_then(get_governance_handler)
        .recover(handle_rejection)
}

fn get_all_governances(
    sender: NodeAPI,
    api_key: Option<String>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "governances")
        .and(warp::get())
        .and(api_key_validation(api_key))
        .and(with_sender(sender))
        .and_then(get_all_governances_handler)
        .recover(handle_rejection)
}

fn post_event_request(
    sender: NodeAPI,
    api_key: Option<String>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "requests")
        .and(warp::post())
        .and(api_key_validation(api_key))
        .and(with_sender(sender))
        .and(with_body())
        .and_then(post_event_request_handler)
        .recover(handle_rejection)
}

fn put_approval(
    sender: NodeAPI,
    api_key: Option<String>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "approvals" / String)
        .and(warp::put())
        //.and(warp::header("X-API-KEY"))
        .and(api_key_validation(api_key))
        .and(with_sender(sender))
        .and(with_body())
        .and_then(put_approval_handler)
        .recover(handle_rejection)
}

fn get_events_of_subject(
    sender: NodeAPI,
    api_key: Option<String>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "subjects" / String / "events")
        .and(warp::get())
        .and(with_sender(sender))
        .and(api_key_validation(api_key))
        .and(warp::query::<GetEventsQuery>())
        .and_then(get_events_of_subject_handler)
        .recover(handle_rejection)
}

fn get_event(
    sender: NodeAPI,
    api_key: Option<String>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "subjects" / String / "events" / u64)
        .and(warp::get())
        .and(with_sender(sender))
        .and(api_key_validation(api_key))
        .and_then(get_event_handler)
        .recover(handle_rejection)
}

fn get_event_properties(
    sender: NodeAPI,
    api_key: Option<String>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "subjects" / String / "events" / u64 / "properties")
        .and(warp::get())
        .and(with_sender(sender))
        .and(api_key_validation(api_key))
        .and_then(get_event_properties_handler)
        .recover(handle_rejection)
}

pub fn with_sender(
    sender: NodeAPI,
) -> impl Filter<Extract = (NodeAPI,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || sender.clone())
}

pub fn api_key_validation(
    api_key: Option<String>,
) -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::header::optional::<String>("x-api-key").and_then(move |key: Option<String>| {
        let inner_key = api_key.clone();
        async move {
            let inner_key = inner_key.clone();
            let Some(inner_key) = inner_key else {
                // API KEY NOT NEEDED
                return Ok(String::from(""));
            };
            let Some(key) = key else {
                return Err(warp::reject::custom(Error::Unauthorized));
            };
            if key == inner_key {
                Ok(key)
            } else {
                Err(warp::reject::custom(Error::Unauthorized))
            }
        }
    })
}

pub fn with_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(ref err) = err.find::<Error>() {
        match err {
            Error::InternalServerError => {
                let mut response = Response::new(String::from("Internal Server Error").into());
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                return Ok(response);
            }
            Error::ExecutionError => {
                let mut response = Response::new(String::from("Execution Error").into());
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                return Ok(response);
            }
            Error::RequestError(error) => {
                let mut response = Response::new(String::from(error).into());
                *response.status_mut() = StatusCode::BAD_REQUEST;
                return Ok(response);
            }
            Error::InvalidParameters => {
                let mut response = Response::new(String::from("Invalid Parameters").into());
                *response.status_mut() = StatusCode::BAD_REQUEST;
                return Ok(response);
            }
            Error::NotEnoughPermissions => {
                let mut response = Response::new(String::from("Not Allowed").into());
                *response.status_mut() = StatusCode::UNAUTHORIZED;
                return Ok(response);
            }
            Error::NotFound => {
                let mut response = Response::new(String::from("Not Found").into());
                *response.status_mut() = StatusCode::NOT_FOUND;
                return Ok(response);
            }
            Error::Unauthorized => {
                let mut response = Response::new(String::from("Unauthorized").into());
                *response.status_mut() = StatusCode::UNAUTHORIZED;
                return Ok(response);
            }
        }
    } else {
        Err(err)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_api_rest() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {});
    }
}
