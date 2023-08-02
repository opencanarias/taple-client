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
    ErrorResponse, TapleRequestResponse,
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
    info(
        license(
            name = "AGPL-3.0-only",
            url = "https://raw.githubusercontent.com/opencanarias/taple-client/main/LICENSE"
        )
    ),
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
    security(),
    tags(
        (name = "Approvals"),
        (name = "Requests"),
        (name = "Subjects"),
        (name = "Others"),
    )
)]
pub struct ApiDoc;

pub async fn serve_swagger(
    full_path: FullPath,
    tail: Tail,
    config: Arc<utoipa_swagger_ui::Config<'static>>,
) -> Result<Box<dyn Reply + 'static>, Rejection> {
    if full_path.as_str() == "/api/doc" {
        return Ok(Box::new(redirect::found(Uri::from_static("/api/doc/"))));
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
