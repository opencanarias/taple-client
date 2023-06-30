use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use std::sync::Arc;
use super::responses::{
    TapleRequestResponse,
    ApprovalResponseBody,
    ApprovalRequestResponse,
    ApprovalEntityResponse,
    ApprovalStateResponse
};
use super::{
    bodys::
    {
        CreateRequestBody, EventRequestBody,
        FactRequestBody, SignedBody,
        SignatureBody, TransferRequestBody, EOLRequestBody
    },
    responses::{
        EventContentResponse,
        SubjectDataResponse,
        TapleRequestStateResponse,
        RequestStateResponse,
        PreauthorizedSubjectsResponse,
        ValidationProofResponse,
        GetProofResponse
    }
};
use super::handlers::{
    __path_get_subjects_handler, __path_get_event_handler,
    __path_get_events_of_subject_handler,
    __path_get_subject_handler,
    __path_patch_approval_handler,
};
use warp::{
    http::Uri,
    hyper::{Response, StatusCode},
    path::{FullPath, Tail},
    Rejection, Reply, redirect,
};

#[derive(OpenApi)]
#[openapi(
    paths(get_subject_handler, 
        get_subjects_handler, get_events_of_subject_handler, get_event_handler, 
        patch_approval_handler, 
    ),
    components(
        schemas(
            SignedBody<EventContentResponse>,
            SignedBody<ApprovalRequestResponse>,
            ApprovalRequestResponse,
            SignedBody<ApprovalResponseBody>,
            ApprovalResponseBody,
            FactRequestBody,
            SignatureBody,
            CreateRequestBody,
            EventRequestBody,
            EventContentResponse,
            SubjectDataResponse,
            TransferRequestBody,
            EOLRequestBody,
            TapleRequestStateResponse,
            RequestStateResponse,
            ApprovalStateResponse,
            ApprovalEntityResponse,
            TapleRequestResponse,
            PreauthorizedSubjectsResponse,
            ValidationProofResponse,
            GetProofResponse
       )
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