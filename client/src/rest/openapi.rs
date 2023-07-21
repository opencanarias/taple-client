use super::handlers::{
    __path_get_allowed_subjects_handler, __path_get_approval_handler, __path_get_approvals_handler,
    __path_get_event_handler, __path_get_events_of_subject_handler, __path_get_subject_handler,
    __path_get_subjects_handler, __path_get_taple_request_handler,
    __path_get_taple_request_state_handler, __path_get_validation_proof_handle,
    __path_patch_approval_handler, __path_post_event_request_handler,
    __path_post_generate_keys_handler, __path_put_allowed_subjects_handler,
};
use super::responses::{
    ApprovalEntityResponse, ApprovalRequestResponse, ApprovalResponseBody, ApprovalStateResponse,
    TapleRequestResponse, ErrorResponse,
};
use super::{
    bodys::{
        AuthorizeSubjectBody, CreateRequestBody, EOLRequestBody, EventRequestBody, FactRequestBody,
        PatchVoteBody, PostEventRequestBodyPreSignature, SignatureBody, SignedRequestBody,
        TransferRequestBody,
    },
    responses::{
        EventContentResponse, GetProofResponse, PreauthorizedSubjectsResponse,
        RequestStateResponse, SignedApprovalRequestResponse, SignedApprovalResponseBody,
        SignedEvent, SubjectDataResponse, TapleRequestStateResponse, ValidationProofResponse,
    },
};
use std::sync::Arc;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use warp::{
    http::Uri,
    hyper::{Response, StatusCode},
    path::{FullPath, Tail},
    redirect, Rejection, Reply,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        get_allowed_subjects_handler,
        get_subjects_handler,
        get_approval_handler,
        get_approvals_handler,
        get_event_handler,
        get_events_of_subject_handler,
        get_subject_handler,
        get_taple_request_handler,
        get_taple_request_state_handler,
        get_validation_proof_handle,
        patch_approval_handler,
        post_event_request_handler,
        post_generate_keys_handler,
        put_allowed_subjects_handler,
    ),
    components(
        schemas(
            SignedRequestBody,
            SignedEvent,
            SignedApprovalRequestResponse,
            SignedApprovalResponseBody,
            ApprovalRequestResponse,
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
            AuthorizeSubjectBody,
            PreauthorizedSubjectsResponse,
            ValidationProofResponse,
            PatchVoteBody,
            GetProofResponse,
            PostEventRequestBodyPreSignature,
            ErrorResponse
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
    if full_path.as_str() == "/api/documentation" {
        return Ok(Box::new(redirect::found(Uri::from_static(
            "/api/documentation/",
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
