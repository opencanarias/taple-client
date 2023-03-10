use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use taple_core::event_request::{CreateRequest, RequestPayload, StateRequest};
use std::sync::Arc;
use taple_core::{
    Acceptance,
    ApprovalResponse,
    ApprovalResponseContent,
    Event,
    SubjectData,
    event_content::{
        EventContent,
        Metadata
    },
    event_request::{
        EventRequest,
        EventRequestType,
        RequestData
    },
    signature::{
        Signature,
        SignatureContent
    },
};
use super::bodys::{
    CreateRequestBody, EventRequestTypeBody, PostEventBody, PostEventRequestBody, PutVoteBody,
    StateRequestBody, Payload, SignatureRequestContent, SignatureRequest
};
use super::handlers::{
    __path_get_all_governances_handler, __path_get_all_subjects_handler, __path_get_event_handler,
    __path_get_event_properties_handler, __path_get_events_of_subject_handler,
    __path_get_governance_handler, __path_get_pending_requests_handler,
    __path_get_single_request_handler, __path_get_subject_handler,
    __path_post_event_request_handler,
    __path_put_approval_handler,
};
use warp::{
    http::Uri,
    hyper::{Response, StatusCode},
    path::{FullPath, Tail},
    Rejection, Reply, redirect,
};

#[derive(OpenApi)]
#[openapi(
    paths(get_single_request_handler, post_event_request_handler, get_subject_handler, 
        get_all_subjects_handler, get_events_of_subject_handler, get_event_handler, 
        get_event_properties_handler, get_pending_requests_handler,
        put_approval_handler, get_all_governances_handler, get_governance_handler
    ),
    components(
        schemas(StateRequestBody, SignatureRequestContent, SignatureRequest, PostEventBody, RequestPayload, CreateRequestBody, CreateRequest, StateRequest, EventRequestTypeBody, RequestData, SubjectData, Acceptance, ApprovalResponse, ApprovalResponseContent, EventRequest, Payload, PostEventRequestBody, PutVoteBody, Event, EventRequestType, Signature, EventContent, SignatureContent, EventRequest, Metadata)
    ),
    modifiers(&SecurityAddon),
    security(),
    tags(
        (name = "Subjects"),
        (name = "Events"),
        (name = "Requests"),
        (name = "Approvals"),
        (name = "Governances")
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // We can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "api_key",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("x-api-key"))),
        )
    }
}

pub async fn serve_swagger(
    full_path: FullPath,
    tail: Tail,
    config: Arc<utoipa_swagger_ui::Config<'static>>,
) -> Result<Box<dyn Reply + 'static>, Rejection> {
    if full_path.as_str() == "/api/doc/ui" {
        return Ok(Box::new(redirect::found(Uri::from_static(
            "/api/doc/ui/",
        ))));
    }

    let path = tail.as_str();
    match utoipa_swagger_ui::serve(path, config) {
        Ok(file) => {
            if let Some(file) = file {
                Ok(Box::new(
                    Response::builder()
                        .header("Content-Type", file.content_type)
                        .body(file.bytes),
                ))
            } else {
                Ok(Box::new(StatusCode::NOT_FOUND))
            }
        }
        Err(error) => Ok(Box::new(
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error.to_string()),
        )),
    }
}