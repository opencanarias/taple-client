use std::{collections::HashSet, str::FromStr};

use serde::Serialize;
use taple_core::{
    crypto::{KeyMaterial, KeyPair},
    event_request::EventRequest,
    identifier::{Derivable, DigestIdentifier},
    signature::Signature,
    Acceptance, EventRequestType, KeyIdentifier,
};
use warp::Rejection;

use taple_core::{ApiError, ApiModuleInterface, NodeAPI};

use crate::{rest::querys::AddKeysQuery, rest::querys::KeyAlgorithms};

use super::{
    bodys::{
        AuthorizeSubjectBody, PostEventRequestBody, PostEventRequestBodyPreSignature, PutVoteBody,
    },
    error::Error,
    querys::{GetAllSubjectsQuery, GetApprovalsQuery, GetEventsOfSubjectQuery},
    responses::{
        ApprovalPetitionDataResponse, EventResponse, SignatureDataResponse, SubjectDataResponse,
        TapleRequestResponse,
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
                "properties": "{\"localizacion\":\"España\",\"temperatura\":10}"
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
    security(("api_key" = [])),
    params( // TODO: HACE FALTA ACTUALIZAR
        ("from" = Option<String>, Query, description = "Id of initial subject"),
        ("quantity" = Option<usize>, Query, description = "Quantity of subjects requested")
    ),
    responses(
        (status = 200, description = "Subjects Data successfully retrieved", body = [SubjectDataResponse],
        example = json!(
            [
                {
                    "subject_id": "J7BgD3dqZ8vO4WEH7-rpWIH-IhMqaSDnuJ3Jb8K6KvL0",
                    "governance_id": "",
                    "sn": 0,
                    "public_key": "E2tlKVr6wA2GZKoSZi_dwIuz2TVUTCCDpOOwiE2SJbWc",
                    "namespace": "",
                    "schema_id": "",
                    "owner": "EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w",
                    "properties": "{\"members\":[{\"description\":\"Sede en España\",\"id\":\"Compañía1\",\"key\":\"EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w\",\"tags\":{}},{\"description\":\"Sede en Inglaterra\",\"id\":\"Compañía2\",\"key\":\"ECQnl-h1vEWmu-ZlPuweR3N1x6SUImyVdPrCLmnJJMyU\",\"tags\":{}}],\"schemas\":[{\"content\":{\"additionalProperties\":false,\"properties\":{\"localizacion\":{\"type\":\"string\"},\"temperatura\":{\"type\":\"integer\"}},\"required\":[\"temperatura\",\"localizacion\"],\"type\":\"object\"},\"id\":\"Prueba\",\"tags\":{}}]}"
                },
                {
                    "subject_id": "JKZgYhPjQdWNWWwkac0wSwqLKoOJsT0QimJmj6zjimWc",
                    "governance_id": "J7BgD3dqZ8vO4WEH7-rpWIH-IhMqaSDnuJ3Jb8K6KvL0",
                    "sn": 0,
                    "public_key": "ELZ_b-kZzdPykcYuRNC2ZZe_2lCTCUoo60GXfR4cuXMw",
                    "namespace": "namespace1",
                    "schema_id": "Prueba",
                    "owner": "EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w",
                    "properties": "{\"localizacion\":\"España\",\"temperatura\":10}"
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
    // TODO: NAMESPACE DECIDIR CÓMO ESPECIFICAR
    let data = node
        .get_subjects("namespace1".into(), parameters.from, parameters.quantity)
        .await
        .map(|s| {
            s.into_iter()
                .map(|x| SubjectDataResponse::from(x))
                .collect::<Vec<SubjectDataResponse>>()
        });
    handle_data(data)
}

#[utoipa::path(
    post,
    path = "/requests",
    tag = "Requests",
    operation_id = "Create a new Event Request",
    context_path = "/api",
    request_body(content = PostEventRequestBody, content_type = "application/json", description = "Event Request type and payload with the associated signature"),
    responses(
        (status = 202, description = "Event Request Created", body = String,
        example = json!(
            {
                "request": {
                    "Create": {
                        "governance_id": "",
                        "schema_id": "",
                        "namespace": "",
                        "payload": {
                            "Json": "{\"members\":[{\"description\":\"Sede en España\",\"id\":\"Compañía1\",\"key\":\"EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w\",\"tags\":{}},{\"description\":\"Sede en Inglaterra\",\"id\":\"Compañía2\",\"key\":\"ECQnl-h1vEWmu-ZlPuweR3N1x6SUImyVdPrCLmnJJMyU\",\"tags\":{}}],\"schemas\":[{\"content\":{\"additionalProperties\":false,\"properties\":{\"localizacion\":{\"type\":\"string\"},\"temperatura\":{\"type\":\"integer\"}},\"required\":[\"temperatura\",\"localizacion\"],\"type\":\"object\"},\"id\":\"Prueba\",\"tags\":{}}]}"
                        }
                    }
                },
                "request_id": "JpxalqMTQcDcLG3dwb8uvcrstJo6pmFEzUwhzi0nGPOA",
                "timestamp": 1671705355,
                "subject_id": "J7BgD3dqZ8vO4WEH7-rpWIH-IhMqaSDnuJ3Jb8K6KvL0",
                "sn": 0
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Conflict"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn post_event_request_handler(
    node: NodeAPI,
    keys: KeyPair,
    body: PostEventRequestBodyPreSignature,
) -> Result<Box<dyn warp::Reply>, Rejection> {
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
        .external_request(EventRequest { request, signature })
        .await
        .map(|id| id.to_str());
    handle_data(data)
}

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

pub async fn post_preauthorized_subjects_handler(
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

/*
#[utoipa::path(
    get,
    path = "/approvals",
    tag = "Approvals",
    operation_id = "Get all the pending requests for Approval",
    context_path = "/api",
    responses(
        (status = 200, description = "All pending requests", body =  [ApprovalPetitionDataResponse],
        example = json!(
            [
                {
                    "request": {
                        "State": {
                            "subject_id": "J7BgD3dqZ8vO4WEH7-rpWIH-IhMqaSDnuJ3Jb8K6KvL0",
                            "payload": {
                                "Json": "{\"members\":[{\"description\":\"Sede en España\",\"id\":\"Compañía1\",\"key\":\"EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w\",\"tags\":{}}],\"schemas\":[{\"content\":{\"additionalProperties\":false,\"properties\":{\"localizacion\":{\"type\":\"string\"},\"temperatura\":{\"type\":\"integer\"}},\"required\":[\"temperatura\",\"localizacion\"],\"type\":\"object\"},\"id\":\"Prueba\",\"tags\":{}}]}"
                            }
                        }
                    },
                    "timestamp": 1671709394,
                    "signature": {
                        "content": {
                            "signer": "EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w",
                            "event_content_hash": "JhEnzFVF1a-u-rH34cix2A_OXgcfesM6HGOyk7wdrGHk",
                            "timestamp": 1671709394
                        },
                        "signature": "SEnUJq3Y1lbmijKzrc0kuLu-FgMTCyo5PWfDrbi_80bspghCny8Yuvifsmdqq0TjfTUS7sEwmOLir1W_1zeIVyDQ"
                    },
                    "approvals": []
                }
            ]
        )),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_pending_requests_handler(
    node: NodeAPI,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let data = node.get_pending_requests().await.map(|result| {
        result
            .into_iter()
            .map(|r| ApprovalPetitionDataResponse::from(r))
            .collect::<Vec<ApprovalPetitionDataResponse>>()
    });
    handle_data(data)
}

#[utoipa::path(
    get,
    path = "/approvals/{id}",
    tag = "Approvals",
    operation_id = "Get a specific pending request for Approval",
    context_path = "/api",
    responses(
        (status = 200, description = "The pending request", body = ApprovalPetitionDataResponse,
        example = json!(
            {
                "request": {
                    "State": {
                        "subject_id": "J7BgD3dqZ8vO4WEH7-rpWIH-IhMqaSDnuJ3Jb8K6KvL0",
                        "payload": {
                            "Json": "{\"members\":[{\"description\":\"Sede en España\",\"id\":\"Compañía1\",\"key\":\"EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w\",\"tags\":{}}],\"schemas\":[{\"content\":{\"additionalProperties\":false,\"properties\":{\"localizacion\":{\"type\":\"string\"},\"temperatura\":{\"type\":\"integer\"}},\"required\":[\"temperatura\",\"localizacion\"],\"type\":\"object\"},\"id\":\"Prueba\",\"tags\":{}}]}"
                        }
                    }
                },
                "timestamp": 1671709394,
                "signature": {
                    "content": {
                        "signer": "EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w",
                        "event_content_hash": "JhEnzFVF1a-u-rH34cix2A_OXgcfesM6HGOyk7wdrGHk",
                        "timestamp": 1671709394
                    },
                    "signature": "SEnUJq3Y1lbmijKzrc0kuLu-FgMTCyo5PWfDrbi_80bspghCny8Yuvifsmdqq0TjfTUS7sEwmOLir1W_1zeIVyDQ"
                },
                "approvals": []
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_single_request_handler(
    id: String,
    node: NodeAPI,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let result = if let Ok(id) = DigestIdentifier::from_str(&id) {
        node.get_single_request(id)
            .await
            .map(|r| ApprovalPetitionDataResponse::from(r))
    } else {
        Err(ApiError::InvalidParameters(format!(
            "ID specified is not a valid Digest Identifier"
        )))
    };
    handle_data(result)
}
*/

pub async fn get_approval_handler(
    id: String,
    node: NodeAPI,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let result = if let Ok(id) = DigestIdentifier::from_str(&id) {
        node.get_approval(id)
            .await
            .map(|r| ApprovalPetitionDataResponse::from(r.0))
    } else {
        Err(ApiError::InvalidParameters(format!(
            "ID specified is not a valid Digest Identifier"
        )))
    };
    handle_data(result)
}

pub async fn get_approvals_handler(
    node: NodeAPI,
    parameters: GetApprovalsQuery,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let data = node.get_approvals(parameters.status).await.map(|result| {
        result
            .into_iter()
            .map(|r| ApprovalPetitionDataResponse::from(r))
            .collect::<Vec<ApprovalPetitionDataResponse>>()
    });
    handle_data(data)
}

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
    put,
    path = "/approvals{id}",
    operation_id = "Set your Aprroval for a request",
    tag = "Approvals",
    context_path = "/api",
    request_body(content = PutVoteBody, content_type = "application/json", description = "Vote of the user for an existing request"),
    params(
        ("id" = String, Path, description = "Request's unique id"),
    ),
    responses(
        (status = 200, description = "Request successfully voted",
        example = json!(
            Option::<String>::None;
        )),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn put_approval_handler(
    request_id: String,
    node: NodeAPI,
    body: PutVoteBody,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let acceptance = match body {
        PutVoteBody::Accept => Acceptance::Ok,
        PutVoteBody::Reject => Acceptance::Ko,
    };
    let result = if let Ok(id) = DigestIdentifier::from_str(&request_id) {
        node.approval_request(id, acceptance)
            .await
            .map(|id| id.to_str())
    } else {
        Err(ApiError::InvalidParameters(format!(
            "ID specified is not a valid Digest Identifier"
        )))
    };
    handle_data(result)
}

#[utoipa::path(
    get,
    path = "/governances/{id}",
    operation_id = "Get Governance Data",
    tag = "Governances",
    context_path = "/api",
    params(
        ("id" = String, Path, description = "Governance's unique id")
    ),
    responses(
        (status = 200, description = "Subject Data successfully retrieved", body = SubjectDataResponse, 
            example = json!(
                {
                    "subject_id": "J7BgD3dqZ8vO4WEH7-rpWIH-IhMqaSDnuJ3Jb8K6KvL0",
                    "governance_id": "",
                    "sn": 0,
                    "public_key": "E2tlKVr6wA2GZKoSZi_dwIuz2TVUTCCDpOOwiE2SJbWc",
                    "namespace": "",
                    "schema_id": "",
                    "owner": "EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w",
                    "properties": "{\"members\":[{\"description\":\"Sede en España\",\"id\":\"Compañía1\",\"key\":\"EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w\",\"tags\":{}},{\"description\":\"Sede en Inglaterra\",\"id\":\"Compañía2\",\"key\":\"ECQnl-h1vEWmu-ZlPuweR3N1x6SUImyVdPrCLmnJJMyU\",\"tags\":{}}],\"schemas\":[{\"content\":{\"additionalProperties\":false,\"properties\":{\"localizacion\":{\"type\":\"string\"},\"temperatura\":{\"type\":\"integer\"}},\"required\":[\"temperatura\",\"localizacion\"],\"type\":\"object\"},\"id\":\"Prueba\",\"tags\":{}}]}"
                }
            )
        ),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_governance_handler(
    id: String,
    node: NodeAPI,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let result = if let Ok(id) = DigestIdentifier::from_str(&id) {
        node.get_subject(id)
            .await
            .map(|s| SubjectDataResponse::from(s))
    } else {
        Err(ApiError::InvalidParameters(format!(
            "ID specified is not a valid Digest Identifier"
        )))
    };
    handle_data(result)
}

#[utoipa::path(
    get,
    path = "/governances",
    operation_id = "Get all Governances data",
    tag = "Governances",
    context_path = "/api",
    params(
        ("from" = Option<String>, Query, description = "Id of initial subject"),
        ("quantity" = Option<usize>, Query, description = "Quantity of subjects requested")
    ),
    responses(
        (status = 200, description = "Subjets Data successfully retrieved", body = [SubjectDataResponse],
        example = json!(
            [
                {
                    "subject_id": "J7BgD3dqZ8vO4WEH7-rpWIH-IhMqaSDnuJ3Jb8K6KvL0",
                    "governance_id": "",
                    "sn": 0,
                    "public_key": "E2tlKVr6wA2GZKoSZi_dwIuz2TVUTCCDpOOwiE2SJbWc",
                    "namespace": "",
                    "schema_id": "",
                    "owner": "EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w",
                    "properties": "{\"members\":[{\"description\":\"Sede en España\",\"id\":\"Compañía1\",\"key\":\"EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w\",\"tags\":{}},{\"description\":\"Sede en Inglaterra\",\"id\":\"Compañía2\",\"key\":\"ECQnl-h1vEWmu-ZlPuweR3N1x6SUImyVdPrCLmnJJMyU\",\"tags\":{}}],\"schemas\":[{\"content\":{\"additionalProperties\":false,\"properties\":{\"localizacion\":{\"type\":\"string\"},\"temperatura\":{\"type\":\"integer\"}},\"required\":[\"temperatura\",\"localizacion\"],\"type\":\"object\"},\"id\":\"Prueba\",\"tags\":{}}]}"
                }
            ]
        )),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_all_governances_handler(
    node: NodeAPI,
    parameters: GetAllSubjectsQuery,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let data = node
        .get_governances("namespace1".into(), parameters.from, parameters.quantity)
        .await
        .map(|result| {
            result
                .into_iter()
                .map(|s| SubjectDataResponse::from(s))
                .collect::<Vec<SubjectDataResponse>>()
        });
    handle_data(data)
}

#[utoipa::path(
    get,
    path = "/subjects/{id}/events",
    operation_id = "Get all Events from indicated Subject",
    context_path = "/api",
    tag = "Events",
    params(
        ("id" = String, Path, description = "Subject's unique id"),
        ("from" = Option<usize>, Query, description = "Initial SN"),
        ("quantity" = Option<usize>, Query, description = "Quantity of events requested"),
    ),
    responses(
        (status = 200, description = "Subjects Data successfully retrieved", body = [EventResponse],
        example = json!(
            [
                {
                    "event_content": {
                        "subject_id": "JKZgYhPjQdWNWWwkac0wSwqLKoOJsT0QimJmj6zjimWc",
                        "event_request": {
                            "request": {
                                "Create": {
                                    "governance_id": "J7BgD3dqZ8vO4WEH7-rpWIH-IhMqaSDnuJ3Jb8K6KvL0",
                                    "schema_id": "Prueba",
                                    "namespace": "namespace1",
                                    "payload": {
                                        "Json": "{\"localizacion\":\"España\",\"temperatura\":10}"
                                    }
                                }
                            },
                            "timestamp": 1671705820,
                            "signature": {
                                "content": {
                                    "signer": "EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w",
                                    "event_content_hash": "J7nteIKy2WcAZE5l_N24A1ORR5YQzZxE5DzyfUdxKxz4",
                                    "timestamp": 1671705820
                                },
                                "signature": "SEO7aEICzPXef0scuC4bBEfYjBxgqAYwP0Lx2x4llCqNYMNuUZ-r5lFJ-PIllZQK_4luo0km_z3LV3MNgaUNPhBQ"
                            },
                            "approvals": []
                        },
                        "sn": 0,
                        "previous_hash": "",
                        "state_hash": "JCtaluRLrMLdXHBJFRUx-_Rj3CWLdU0KIAdS-Jumf4eQ",
                        "metadata": {
                            "namespace": "namespace1",
                            "governance_id": "J7BgD3dqZ8vO4WEH7-rpWIH-IhMqaSDnuJ3Jb8K6KvL0",
                            "governance_version": 0,
                            "schema_id": "Prueba",
                            "owner": "EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w"
                        },
                        "approved": true
                    },
                    "signature": {
                        "content": {
                            "signer": "ELZ_b-kZzdPykcYuRNC2ZZe_2lCTCUoo60GXfR4cuXMw",
                            "event_content_hash": "JnMRtYtb2DD2cueHe4oAVMJUoqtkexwZa_n6WWFmH8eA",
                            "timestamp": 1671705820
                        },
                        "signature": "SEWJtxV6gvX-ORUZgs8geu_7wLmZVg23oQVzqtFt-JQJrZRjc_Uil8_J_9atO9skgpFupiFCp8ZEPgVSF9aQJHDA"
                    }
                },
                {
                    "event_content": {
                        "subject_id": "JKZgYhPjQdWNWWwkac0wSwqLKoOJsT0QimJmj6zjimWc",
                        "event_request": {
                            "request": {
                                "State": {
                                    "subject_id": "JKZgYhPjQdWNWWwkac0wSwqLKoOJsT0QimJmj6zjimWc",
                                    "payload": {
                                        "Json": "{\"localizacion\":\"Argentina\",\"temperatura\":-3}"
                                    }
                                }
                            },
                            "timestamp": 1671706794,
                            "signature": {
                                "content": {
                                    "signer": "EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w",
                                    "event_content_hash": "JBmfwxOtP2gXFzyTQX0NzVw8ByiHjxcyBgaBamYoOhcA",
                                    "timestamp": 1671706794
                                },
                                "signature": "SEuYCV5T0G4Vpps859QQMzimXw8NcYailkXwh2oKtsVX82iJQzbspKR7nLllcHiKfuWRkzCWbFpQzxPBWdsuZgBA"
                            },
                            "approvals": []
                        },
                        "sn": 1,
                        "previous_hash": "JnMRtYtb2DD2cueHe4oAVMJUoqtkexwZa_n6WWFmH8eA",
                        "state_hash": "JMqLbPz7VY1pjuj9-n0qT0UuOGH_TpQVRaVEOHSaE_5Y",
                        "metadata": {
                            "namespace": "namespace1",
                            "governance_id": "J7BgD3dqZ8vO4WEH7-rpWIH-IhMqaSDnuJ3Jb8K6KvL0",
                            "governance_version": 0,
                            "schema_id": "Prueba",
                            "owner": "EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w"
                        },
                        "approved": true
                    },
                    "signature": {
                        "content": {
                            "signer": "ELZ_b-kZzdPykcYuRNC2ZZe_2lCTCUoo60GXfR4cuXMw",
                            "event_content_hash": "JYkgipgpilkVFVV_hJ0Dxvr2eHmXU6niKTmKSjMVYEZY",
                            "timestamp": 1671706794
                        },
                        "signature": "SEDcHW5nM7HPsQJyHQkAVaV5NkTuwT2fJL_T9r0HmqbgT3Wt7AMTFpjJNunlCSa-dEosItNu5P9k05vAE9064TBg"
                    }
                }
            ]
        )),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_events_of_subject_handler(
    id: String,
    node: NodeAPI,
    parameters: GetEventsOfSubjectQuery,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let result = if let Ok(id) = DigestIdentifier::from_str(&id) {
        node.get_events(id, parameters.from, parameters.quantity)
            .await
            .map(|ve| {
                ve.into_iter()
                    .map(|e| EventResponse::try_from(e).unwrap())
                    .collect::<Vec<EventResponse>>()
            })
    } else {
        Err(ApiError::InvalidParameters(format!(
            "ID specified is not a valid Digest Identifier"
        )))
    };
    handle_data::<Vec<EventResponse>>(result)
}

#[utoipa::path(
    get,
    path = "/subjects/{id}/events/{sn}",
    operation_id = "Get the Event data of indicated Events",
    tag = "Events",
    context_path = "/api",
    params(
        ("id" = String, Path, description = "Subject's unique id"),
        ("sn" = u64, Path, description = "Event sn"),
    ),
    responses(
        (status = 200, description = "Subjects Data successfully retrieved", body = EventResponse,
        example = json!(
            {
                "event_content": {
                    "subject_id": "JKZgYhPjQdWNWWwkac0wSwqLKoOJsT0QimJmj6zjimWc",
                    "event_request": {
                        "request": {
                            "State": {
                                "subject_id": "JKZgYhPjQdWNWWwkac0wSwqLKoOJsT0QimJmj6zjimWc",
                                "payload": {
                                    "Json": "{\"localizacion\":\"Argentina\",\"temperatura\":-3}"
                                }
                            }
                        },
                        "timestamp": 1671706794,
                        "signature": {
                            "content": {
                                "signer": "EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w",
                                "event_content_hash": "JBmfwxOtP2gXFzyTQX0NzVw8ByiHjxcyBgaBamYoOhcA",
                                "timestamp": 1671706794
                            },
                            "signature": "SEuYCV5T0G4Vpps859QQMzimXw8NcYailkXwh2oKtsVX82iJQzbspKR7nLllcHiKfuWRkzCWbFpQzxPBWdsuZgBA"
                        },
                        "approvals": []
                    },
                    "sn": 1,
                    "previous_hash": "JnMRtYtb2DD2cueHe4oAVMJUoqtkexwZa_n6WWFmH8eA",
                    "state_hash": "JMqLbPz7VY1pjuj9-n0qT0UuOGH_TpQVRaVEOHSaE_5Y",
                    "metadata": {
                        "namespace": "namespace1",
                        "governance_id": "J7BgD3dqZ8vO4WEH7-rpWIH-IhMqaSDnuJ3Jb8K6KvL0",
                        "governance_version": 0,
                        "schema_id": "Prueba",
                        "owner": "EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w"
                    },
                    "approved": true
                },
                "signature": {
                    "content": {
                        "signer": "ELZ_b-kZzdPykcYuRNC2ZZe_2lCTCUoo60GXfR4cuXMw",
                        "event_content_hash": "JYkgipgpilkVFVV_hJ0Dxvr2eHmXU6niKTmKSjMVYEZY",
                        "timestamp": 1671706794
                    },
                    "signature": "SEDcHW5nM7HPsQJyHQkAVaV5NkTuwT2fJL_T9r0HmqbgT3Wt7AMTFpjJNunlCSa-dEosItNu5P9k05vAE9064TBg"
                }
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
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

pub async fn get_validation_proof_handle(
    id: String,
    node: NodeAPI,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let result = if let Ok(id) = DigestIdentifier::from_str(&id) {
        node.get_validation_proof(id).await.map(|r| {
            r.into_iter()
                .map(|s| SignatureDataResponse::from(s))
                .collect::<Vec<SignatureDataResponse>>()
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
