use std::{collections::HashSet, str::FromStr};

use serde::Serialize;
use taple_core::{
    crypto::{KeyMaterial, KeyPair},
    identifier::{Derivable, DigestIdentifier},
    signature::{Signature, Signed},
    ApprovalState, KeyDerivator, KeyIdentifier,
};
use warp::Rejection;

use taple_core::{ApiError, ApiModuleInterface, NodeAPI};

use crate::{rest::querys::AddKeysQuery, rest::querys::KeyAlgorithms};
use crate::rest::querys::GetWithPaginationString;

use super::{
    bodys::{
        AuthorizeSubjectBody, PatchVoteBody, PostEventRequestBodyPreSignature, SignatureBody,
        SignedBody,
    },
    error::Error,
    querys::{GetAllSubjectsQuery, GetApprovalsQuery, GetWithPagination},
    responses::{
        ApprovalEntityResponse, EventContentResponse, GetProofResponse,
        PreauthorizedSubjectsResponse, SubjectDataResponse, TapleRequestResponse,
        TapleRequestStateResponse, ValidationProofResponse,
    },
};

#[utoipa::path(
    get,
    path = "/subjects/{id}",
    operation_id = "Get Subject Data",
    tag = "Subjects",
    context_path = "/api",
    params(
        ("id" = String, Path, description = "Subject's unique id")
    ),
    responses(
        (status = 200, description = "Subject Data successfully retrieved", body = SubjectDataResponse,
        example = json!( 
            {
                "subject_id": "JKZgYhPjQdWNWWwkac0wSwqLKoOJsT0QimJmj6zjimWc",
                "governance_id": "J7BgD3dqZ8vO4WEH7-rpWIH-IhMqaSDnuJ3Jb8K6KvL0",
                "sn": 0,
                "public_key": "ELZ_b-kZzdPykcYuRNC2ZZe_2lCTCUoo60GXfR4cuXMw",
                "namespace": "namespace1",
                "schema_id": "Prueba",
                "owner": "EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w",
                "properties": {
                    "localizacion": "España",
                    "temperatura": 10
                },
                "name": "pedro",
                "creator": "EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w"
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_subject_handler(
    id: String,
    node: NodeAPI,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let response = if let Ok(id) = DigestIdentifier::from_str(&id) {
        node.get_subject(id)
            .await
            .map(|s| SubjectDataResponse::from(s))
    } else {
        Err(ApiError::InvalidParameters(format!(
            "ID specified is not a valid Digest Identifier"
        )))
    };
    handle_data(response)
}

#[utoipa::path(
    get,
    path = "/subjects",
    tag = "Subjects",
    operation_id = "Get All Subjects Data",
    context_path = "/api",
    params( // TODO: HACE FALTA ACTUALIZAR
        ("from" = Option<String>, Query, description = "Id of initial subject"),
        ("quantity" = Option<isize>, Query, description = "Quantity of subjects requested"),
        ("subject_type" = Option<String>, Query, description = "Type of subjects requested"),
        ("governanceid" = Option<String>, Query, description = "Governance id of subjects requested")
    ),
    responses(
        (status = 200, description = "Subjects Data successfully retrieved", body = [SubjectDataResponse],
        example = json!(
            [
                {
                    "subject_id": "JKZgYhPjQdWNWWwkac0wSwqLKoOJsT0QimJmj6zjimWc",
                    "governance_id": "J7BgD3dqZ8vO4WEH7-rpWIH-IhMqaSDnuJ3Jb8K6KvL0",
                    "sn": 0,
                    "public_key": "ELZ_b-kZzdPykcYuRNC2ZZe_2lCTCUoo60GXfR4cuXMw",
                    "namespace": "namespace1",
                    "schema_id": "Prueba",
                    "owner": "EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w",
                    "properties": {
                        "localizacion": "España",
                        "temperatura": 10
                    },
                    "name": "pedro",
                    "creator": "EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w"
                }
            ]
        )),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_subjects_handler(
    node: NodeAPI,
    parameters: GetAllSubjectsQuery,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    enum SubjectType {
        All,
        Governances,
    }

    let subject_type = match &parameters.subject_type {
        Some(data) => match data.to_lowercase().as_str() {
            "all" => SubjectType::All,
            "governances" => {
                if let Some(_) = &parameters.governanceid {
                    return handle_data::<SubjectDataResponse>(Err(ApiError::InvalidParameters(
                        format!("governanceid can not be specified with subject_type=governances"),
                    )));
                }
                SubjectType::Governances
            }
            other => {
                return handle_data::<SubjectDataResponse>(Err(ApiError::InvalidParameters(
                    format!("unknow parameter {}", other),
                )));
            }
        },
        None => SubjectType::All,
    };

    let data = match subject_type {
        SubjectType::All => {
            if let Some(data) = &parameters.governanceid {
                match DigestIdentifier::from_str(data) {
                    Ok(id) => {
                        node.get_subjects_by_governance(id, parameters.from, parameters.quantity)
                            .await
                    }
                    Err(_) => Err(ApiError::InvalidParameters("governanceid".to_owned())),
                }
            } else {
                node.get_subjects("".into(), parameters.from, parameters.quantity)
                    .await
            }
        }
        SubjectType::Governances => {
            node.get_governances("".into(), parameters.from, parameters.quantity)
                .await
        }
    }
    .map(|s| {
        s.into_iter()
            .map(|x| SubjectDataResponse::from(x))
            .collect::<Vec<SubjectDataResponse>>()
    });
    handle_data(data)
}

#[utoipa::path(
    post,
    path = "/event-requests",
    tag = "Requests",
    operation_id = "createEventRequest",
    context_path = "/api",
    request_body = PostEventRequestBodyPreSignature,
    responses(
        (status = 201, description = "Request Created Successfully", body = String,
        example = json!(
            "JKZgYhPjQdWNWWwkac0wSwqLKoOJsT0QimJmj6zjimWc"
        )),
        (status = 400, description = "Bad Request"),
        (status = 409, description = "Conflict"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn post_event_request_handler(
    node: NodeAPI,
    keys: KeyPair,
    derivator: KeyDerivator,
    mut body: PostEventRequestBodyPreSignature,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    // If event request is a creation one and it does not specify a public_key, then a random one must be generated
    if let super::bodys::EventRequestBody::Create(creation_req) = &mut body.request {
        if creation_req.public_key.is_none() {
            let public_key = node.add_keys(derivator).await;
            if public_key.is_err() {
                return handle_data(public_key);
            }
            let public_key = public_key.unwrap().to_str();
            creation_req.public_key = Some(public_key);
        }
    }
    let signer = KeyIdentifier::new(keys.get_key_derivator(), &keys.public_key_bytes());
    let Ok(request) = body.request.try_into() else {
        return Err(warp::reject::custom(
            Error::InvalidParameters("Invalid request".to_owned()),
        ));
    };
    let signature = match body.signature {
        Some(signature) => {
            let signature = signature.try_into();
            if signature.is_err() {
                return handle_data(signature);
            } else {
                signature.unwrap()
            }
        }
        None => Signature::new(&request, signer, &keys).expect("Error signing request"),
    };
    let data = node
        .external_request(Signed {
            content: request,
            signature,
        })
        .await
        .map(|id| id.to_str());
    handle_data(data)
}

#[utoipa::path(
    post,
    path = "/keys",
    tag = "Keys",
    operation_id = "createKeys",
    context_path = "/api",
    responses(
        (status = 201, description = "KeyPair Created Successfully", body = String,
        example = json!(
            "ELZ_b-kZzdPykcYuRNC2ZZe_2lCTCUoo60GXfR4cuXMw"
        )),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn post_generate_keys_handler(
    node: NodeAPI,
    parameters: AddKeysQuery,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let derivator = parameters
        .algorithm
        .unwrap_or(KeyAlgorithms::Ed25519)
        .into();
    let result = node.add_keys(derivator).await;
    handle_data(result)
}

#[utoipa::path(
    get,
    path = "/allowed-subjects",
    operation_id = "Get Allowed Subject Data",
    tag = "getAllowedSubjects",
    context_path = "/api",
    params( 
        ("from" = Option<String>, Query, description = "Id of initial subject"),
        ("quantity" = Option<isize>, Query, description = "Quantity of subjects requested")
    ),
    responses(
        (status = 200, description = "Subject Data successfully retrieved", body = SubjectDataResponse,
        example = json!( 
            [
                    {
                        "subject_id": "JKZgYhPjQdWNWWwkac0wSwqLKoOJsT0QimJmj6zjimWc",
                        "providers": []
                    }
                ]
        )),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_allowed_subjects_handler(
    node: NodeAPI,
    parameters: GetWithPaginationString,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let result = node
        .get_all_allowed_subjects_and_providers(parameters.from, parameters.quantity)
        .await
        .map(|x| {
            Vec::from_iter(
                x.into_iter()
                    .map(|s| PreauthorizedSubjectsResponse::from(s)),
            )
        });
    handle_data(result)
}

#[utoipa::path(
    put,
    path = "/allowed-subjects/{id}",
    operation_id = "Put Allowed Subject Data",
    tag = "putAllowedSubjects",
    context_path = "/api",
    params(
        ("id" = String, Path, description = "Subject's unique id")
    ),
    request_body = AuthorizeSubjectBody,
    responses(
        (status = 201, description = "Subject Data successfully created", body = AuthorizeSubjectBody,
        example = json!( 
            [
                    {
                        "subject_id": "JKZgYhPjQdWNWWwkac0wSwqLKoOJsT0QimJmj6zjimWc",
                        "providers": []
                    }
                ]
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn put_allowed_subjects_handler(
    id: String,
    node: NodeAPI,
    body: AuthorizeSubjectBody,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let result = 'result: {
        let subject_id = match DigestIdentifier::from_str(&id) {
            Ok(subject_id) => subject_id,
            Err(_error) => {
                break 'result Err(ApiError::InvalidParameters(format!(
                    "Invalid digest identifier {}",
                    id
                )))
            }
        };
        let mut providers = HashSet::new();
        for provider in body.providers.iter() {
            let provider = match KeyIdentifier::from_str(provider) {
                Ok(provider) => provider,
                Err(_error) => {
                    break 'result Err(ApiError::InvalidParameters(format!(
                        "Invalid key identifier {}",
                        provider
                    )))
                }
            };
            providers.insert(provider);
        }
        if let Err(error) = node.add_preauthorize_subject(&subject_id, &providers).await {
            break 'result Err(error);
        };
        Ok(())
    };
    handle_data(result.map(|_| body))
}

#[utoipa::path(
    get,
    path = "/approval-requests/{id}",
    operation_id = "Get Approval Request Data",
    tag = "getApprovalRequest",
    context_path = "/api",
    params(
        ("id" = String, Path, description = "Approval's unique id")
    ),
    responses(
        (status = 200, description = "Approval Data successfully retrieved", body = ApprovalEntityResponse),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_approval_handler(
    id: String,
    node: NodeAPI,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let result = if let Ok(id) = DigestIdentifier::from_str(&id) {
        node.get_approval(id)
            .await
            .map(|r| ApprovalEntityResponse::from(r))
    } else {
        Err(ApiError::InvalidParameters(format!(
            "ID specified is not a valid Digest Identifier"
        )))
    };
    handle_data(result)
}

#[utoipa::path(
    get,
    path = "/approval-requests",
    operation_id = "Get Approval Request Data",
    tag = "getApprovalRequests",
    context_path = "/api",
    params(
        ("id" = String, Path, description = "Approval's unique id"),
        ("status" = Option<String>, Query, description = "Approval's status"),
        ("from" = Option<String>, Query, description = "Id of initial approval"),
        ("quantity" = Option<isize>, Query, description = "Quantity of approvals requested"
    )),
    responses(
        (status = 200, description = "Approvals Data successfully retrieved", body = [ApprovalEntityResponse]),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_approvals_handler(
    node: NodeAPI,
    parameters: GetApprovalsQuery,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let status = match parameters.status {
        None => None,
        Some(value) => match value.to_lowercase().as_str() {
            "pending" => Some(ApprovalState::Pending),
            "obsolete" => Some(ApprovalState::Obsolete),
            "responded" => Some(ApprovalState::Responded),
            other => {
                return handle_data::<Vec<ApprovalEntityResponse>>(Err(
                    ApiError::InvalidParameters(format!("status={}", other)),
                ))
            }
        },
    };
    let data = node
        .get_approvals(status, parameters.from, parameters.quantity)
        .await
        .map(|result| {
            result
                .into_iter()
                .map(|r| ApprovalEntityResponse::from(r))
                .collect::<Vec<ApprovalEntityResponse>>()
        });
    handle_data(data)
}

#[utoipa::path(
    get,
    path = "/event-requests/{id}",
    operation_id = "Get Event Request Data",
    tag = "getEventRequest",
    context_path = "/api",
    params(
        ("id" = String, Path, description = "Event Request's unique id"),),
    responses(
        (status = 200, description = "Request Data successfully retrieved", body = TapleRequestResponse),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_taple_request_handler(
    request_id: String,
    node: NodeAPI,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let result = if let Ok(id) = DigestIdentifier::from_str(&request_id) {
        node.get_request(id)
            .await
            .map(|data| TapleRequestResponse::from(data))
    } else {
        Err(ApiError::InvalidParameters(format!(
            "ID specified is not a valid Digest Identifier"
        )))
    };
    handle_data(result)
}

#[utoipa::path(
    get,
    path = "/event-requests/{id}/state",
    operation_id = "Get Event Request State Data",
    tag = "getEventRequestState",
    context_path = "/api",
    params(
        ("id" = String, Path, description = "Event Request's unique id"),),
    responses(
        (status = 200, description = "Request Data successfully retrieved", body = TapleRequestStateResponse),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_taple_request_state_handler(
    request_id: String,
    node: NodeAPI,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let result = if let Ok(id) = DigestIdentifier::from_str(&request_id) {
        node.get_request(id)
            .await
            .map(|data| TapleRequestStateResponse::from(data))
    } else {
        Err(ApiError::InvalidParameters(format!(
            "ID specified is not a valid Digest Identifier"
        )))
    };
    handle_data(result)
}

#[utoipa::path(
    patch,
    path = "/approval-requests/{id}",
    operation_id = "Set your Aprroval for a request",
    tag = "patchApproval",
    context_path = "/api",
    request_body(content = PatchVoteBody, content_type = "application/json", description = "Vote of the user for an existing request"),
    params(
        ("id" = String, Path, description = "Approval's unique id"),
    ),
    responses(
        (status = 200, description = "Request successfully voted", body = ApprovalEntityResponse),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn patch_approval_handler(
    request_id: String,
    node: NodeAPI,
    body: PatchVoteBody,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let acceptance = match body {
        PatchVoteBody::Accept => true,
        PatchVoteBody::Reject => false,
    };
    let result = if let Ok(id) = DigestIdentifier::from_str(&request_id) {
        node.approval_request(id, acceptance)
            .await
            .map(|data| ApprovalEntityResponse::from(data))
    } else {
        Err(ApiError::InvalidParameters(format!(
            "ID specified is not a valid Digest Identifier"
        )))
    };
    handle_data(result)
}

#[utoipa::path(
    get,
    path = "/subjects/{id}/events",
    operation_id = "getEvents",
    context_path = "/api",
    tag = "Events",
    params(
        ("id" = String, Path, description = "Subject's unique id"),
        ("from" = Option<usize>, Query, description = "Initial SN"),
        ("quantity" = Option<usize>, Query, description = "Quantity of events requested"),
    ),
    responses(
        (status = 200, description = "Subjects Data successfully retrieved", body = [SignedBody<EventContentResponse>]),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_events_of_subject_handler(
    id: String,
    node: NodeAPI,
    parameters: GetWithPagination,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let result = if let Ok(id) = DigestIdentifier::from_str(&id) {
        node.get_events(id, parameters.from, parameters.quantity)
            .await
            .map(|ve| {
                ve.into_iter()
                    .map(|e| SignedBody::<EventContentResponse>::from(e))
                    .collect::<Vec<SignedBody<EventContentResponse>>>()
            })
    } else {
        Err(ApiError::InvalidParameters(format!(
            "ID specified is not a valid Digest Identifier"
        )))
    };
    handle_data::<Vec<SignedBody<EventContentResponse>>>(result)
}

#[utoipa::path(
    get,
    path = "/subjects/{id}/events/{sn}",
    operation_id = "getEvent",
    tag = "Events",
    context_path = "/api",
    params(
        ("id" = String, Path, description = "Subject's unique id"),
        ("sn" = u64, Path, description = "Event sn"),
    ),
    responses(
        (status = 200, description = "Subjects Data successfully retrieved", body = SignedBody<EventContentResponse>),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_event_handler(
    id: String,
    sn: u64,
    node: NodeAPI,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let response = if let Ok(id) = DigestIdentifier::from_str(&id) {
        node.get_event(id, sn).await
    } else {
        Err(ApiError::InvalidParameters(format!(
            "ID specified is not a valid Digest Identifier"
        )))
    };
    handle_data(response)
}

#[utoipa::path(
    get,
    path = "/subjects/{id}/validation",
    operation_id = "getValidation",
    tag = "Events",
    context_path = "/api",
    params(
        ("id" = String, Path, description = "Subject's unique id"),
    ),
    responses(
        (status = 200, description = "Subjects Data successfully retrieved", body = GetProofResponse),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_validation_proof_handle(
    id: String,
    node: NodeAPI,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let result = if let Ok(id) = DigestIdentifier::from_str(&id) {
        node.get_validation_proof(id)
            .await
            .map(|(signatures, proof)| GetProofResponse {
                proof: ValidationProofResponse::from(proof),
                signatures: signatures
                    .into_iter()
                    .map(|s| SignatureBody::from(s))
                    .collect::<Vec<SignatureBody>>(),
            })
    } else {
        Err(ApiError::InvalidParameters(format!(
            "ID specified is not a valid Digest Identifier"
        )))
    };
    handle_data(result)
}

pub async fn get_governance_subjects_handle(
    id: String,
    node: NodeAPI,
    parameters: GetAllSubjectsQuery,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let result = if let Ok(id) = DigestIdentifier::from_str(&id) {
        node.get_governance_subjects(id, parameters.from, parameters.quantity)
            .await
            .map(|r| {
                r.into_iter()
                    .map(|s| SubjectDataResponse::from(s))
                    .collect::<Vec<SubjectDataResponse>>()
            })
    } else {
        Err(ApiError::InvalidParameters(format!(
            "ID specified is not a valid Digest Identifier"
        )))
    };
    handle_data(result)
}

pub fn handle_data<T: Serialize + std::fmt::Debug>(
    data: Result<T, ApiError>,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    match &data {
        Ok(data) => return Ok(Box::new(warp::reply::json(&data))),
        Err(ApiError::InvalidParameters(msg)) => Err(warp::reject::custom(
            Error::InvalidParameters(msg.to_owned()),
        )),
        Err(ApiError::NotFound(msg)) => Err(warp::reject::custom(Error::NotFound(msg.to_owned()))),
        Err(ApiError::EventCreationError { .. }) => {
            Err(warp::reject::custom(Error::ExecutionError {
                source: data.unwrap_err(),
            }))
        }
        Err(ApiError::NotEnoughPermissions(_)) => {
            Err(warp::reject::custom(Error::NotEnoughPermissions))
        }
        // Err(ApiError::VoteNotNeeded(msg)) => Err(warp::reject::custom(Error::RequestError(msg.to_owned()))),
        Err(error) => Err(warp::reject::custom(Error::ExecutionError {
            source: error.to_owned(),
        })),
    }
}
