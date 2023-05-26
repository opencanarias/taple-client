use super::handlers::{
    get_all_governances_handler, get_all_subjects_handler, get_event_handler,
    get_events_of_subject_handler, get_governance_handler, get_pending_requests_handler,
    get_single_request_handler, get_subject_handler, post_event_request_handler,
    post_preauthorized_subjects_handler, put_approval_handler, post_expecting_transfer_handler
};
use super::{
    error::Error,
    querys::{GetAllSubjectsQuery, GetEventsOfSubjectQuery},
};
use serde::de::DeserializeOwned;
use taple_core::NodeAPI;
use warp::{hyper::StatusCode, reply::Response, Filter, Rejection, Reply};

pub fn routes(sender: NodeAPI) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_subject(sender.clone())
        .or(get_all_subjects(sender.clone()))
        .or(get_all_governances(sender.clone()))
        .or(get_subject(sender.clone()))
        .or(post_event_request(sender.clone()))
        .or(get_governance(sender.clone()))
        .or(get_events_of_subject(sender.clone()))
        .or(get_event(sender.clone()))
        .or(put_approval(sender.clone()))
        .or(get_single_request(sender.clone()))
        .or(get_pending_requests(sender.clone()))
        .or(post_preauthorized_subjects(sender.clone()))
        .or(post_expecting_transfer(sender.clone()))
}

pub fn get_single_request(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "approvals" / String)
        .and(warp::get())
        .and(with_sender(sender))
        .and_then(get_single_request_handler)
        .recover(handle_rejection)
}

pub fn get_pending_requests(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "approvals")
        .and(warp::get())
        .and(with_sender(sender))
        .and_then(get_pending_requests_handler)
        .recover(handle_rejection)
}

pub fn get_subject(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "subjects" / String)
        .and(warp::get())
        .and(with_sender(sender))
        .and_then(get_subject_handler)
        .recover(handle_rejection)
}

pub fn get_all_subjects(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "subjects")
        .and(warp::get())
        .and(with_sender(sender))
        .and(warp::query::<GetAllSubjectsQuery>())
        .and_then(get_all_subjects_handler)
        .recover(handle_rejection)
}

pub fn get_governance(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "governances" / String)
        .and(warp::get())
        .and(with_sender(sender))
        .and_then(get_governance_handler)
        .recover(handle_rejection)
}

pub fn get_all_governances(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "governances")
        .and(warp::get())
        .and(with_sender(sender))
        .and(warp::query::<GetAllSubjectsQuery>())
        .and_then(get_all_governances_handler)
        .recover(handle_rejection)
}

pub fn post_event_request(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "requests")
        .and(warp::post())
        .and(with_sender(sender))
        .and(with_body())
        .and_then(post_event_request_handler)
        .recover(handle_rejection)
}

pub fn put_approval(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "approvals" / String)
        .and(warp::put())
        //.and(warp::header("X-API-KEY"))
        .and(with_sender(sender))
        .and(with_body())
        .and_then(put_approval_handler)
        .recover(handle_rejection)
}

pub fn post_preauthorized_subjects(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "subjects" / "authorize")
        .and(warp::post())
        .and(with_sender(sender))
        .and(with_body())
        .and_then(post_preauthorized_subjects_handler)
        .recover(handle_rejection)
}

pub fn post_expecting_transfer(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "transfer")
        .and(warp::post())
        .and(with_sender(sender))
        .and(with_body())
        .and_then(post_expecting_transfer_handler)
        .recover(handle_rejection)
}

pub fn get_events_of_subject(
    sender: NodeAPI,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "subjects" / String / "events")
        .and(warp::get())
        .and(with_sender(sender))
        .and(warp::query::<GetEventsOfSubjectQuery>())
        .and_then(get_events_of_subject_handler)
        .recover(handle_rejection)
}

pub fn get_event(sender: NodeAPI) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "subjects" / String / "events" / u64)
        .and(warp::get())
        .and(with_sender(sender))
        .and_then(get_event_handler)
        .recover(handle_rejection)
}

pub fn with_sender(
    sender: NodeAPI,
) -> impl Filter<Extract = (NodeAPI,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || sender.clone())
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
            Error::ExecutionError { .. } => {
                let mut response = Response::new(format!("{}", err).into());
                *response.status_mut() = StatusCode::CONFLICT;
                return Ok(response);
            }
            Error::InvalidParameters(_) => {
                let mut response = Response::new(format!("{}", err).into());
                *response.status_mut() = StatusCode::BAD_REQUEST;
                return Ok(response);
            }
            Error::NotEnoughPermissions => {
                let mut response = Response::new(
                    String::from("Not Allowed. The node does not have the permissions to perform the requested operation."
                ).into());
                *response.status_mut() = StatusCode::FORBIDDEN;
                return Ok(response);
            }
            Error::NotFound(_) => {
                let mut response = Response::new(format!("{}", err).into());
                *response.status_mut() = StatusCode::NOT_FOUND;
                return Ok(response);
            }
            Error::Unauthorized => {
                let mut response = Response::new(format!("{}", err).into());
                *response.status_mut() = StatusCode::UNAUTHORIZED;
                return Ok(response);
            }
            Error::BadRequest(msg) => {
                let mut response = Response::new(msg.to_owned().into());
                *response.status_mut() = StatusCode::BAD_REQUEST;
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
