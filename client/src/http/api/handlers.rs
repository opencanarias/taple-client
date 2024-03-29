use std::{collections::HashSet, str::FromStr};

use serde::Serialize;
use serde_json::Value;
use taple_core::{
    crypto::KeyPair,
    identifier::{Derivable, DigestIdentifier},
    signature::{Signature, Signed},
    ApprovalState, KeyDerivator, KeyIdentifier, DigestDerivator,
};
use warp::Rejection;

use taple_core::{Api, ApiError};

use crate::http::api::querys::GetWithPaginationString;
use crate::{http::api::querys::AddKeysQuery, http::api::querys::KeyAlgorithms};

use super::{
    bodys::{
        self, AuthorizeSubjectBody, PatchVoteBody, PostEventRequestBodyPreSignature, SignatureBody,
        SignedBody,
    },
    error::Error,
    querys::{GetAllSubjectsQuery, GetApprovalsQuery, GetWithPagination},
    responses::{
        ApprovalEntityResponse, EventContentResponse, GetProofResponse,
        PreauthorizedSubjectsResponse, SignedEvent, SubjectDataResponse, TapleRequestResponse,
        TapleRequestStateResponse, ValidationProofResponse,
    },
};

/// Get approvals
///
/// Allows to obtain the list of requests for approvals received by the node.
/// It can also be used, by means of the "status" parameter, to list the requests pending approval.
#[utoipa::path(
    get,
    path = "/approval-requests",
    operation_id = "Get Approval Request Data",
    tag = "Approvals",
    context_path = "/api",
    params(
        ("id" = String, Path, description = "Approval's unique id"),
        ("status" = Option<String>, Query, description = "Approval's status (possibilities: pending, obsolete, responded)"),
        ("from" = Option<String>, Query, description = "Id of initial approval"),
        ("quantity" = Option<isize>, Query, description = "Quantity of approvals requested"
    )),
    responses(
        (status = 200, description = "Approvals Data successfully retrieved", body = [ApprovalEntityResponse],
        example = json!(
            [
                {
                    "id": "J5dfpH-ahrqSo-od4jyZkubyO-XWFJSQ9maK73jKI4Ao",
                    "request": {
                        "event_request": {
                            "Fact": {
                                "subject_id": "JoifaSpfenD2bEPeBLvUTWh30brm4tKcvdW8exQnkGoQ",
                                "payload": {
                                    "Patch": {
                                        "data": [
                                            {
                                                "op": "add",
                                                "path": "/members/0",
                                                "value": {
                                                    "id": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                                                    "name": "WPO"
                                                }
                                            }
                                        ]
                                    }
                                }
                            },
                            "signature": {
                                "signer": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                                "timestamp": 168864358,
                                "value": "SE4yS1Q1Smhm3Az3r6WNFKAGd2Us69vyUA3j5q_riE6MICh_Ub2fSLxNS3Nn-g_CpppvABq6s_c8dF5kbmUir4Ag"
                            }
                        },
                        "sn": 1,
                        "gov_version": 0,
                        "patch": [
                            {
                                "op": "add",
                                "path": "/members/0",
                                "value": {
                                    "id": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                                    "name": "WPO"
                                }
                            }
                        ],
                        "state_hash": "JbDVCZxkDkZ5gLCc7Ge5X75pHHf8dA7_s8UynsnzG5o8",
                        "hash_prev_event": "JLic8SLrT7tJxA9B3aLaaKaIEuV7Wouo2ogHCid6O4g8",
                        "signature": {
                            "signer": "E0gaiDcPRVmYLUGbseHmBk2_2H-FAlSgaO6ZMOXhh4Gs",
                            "timestamp": 168864358,
                            "value": "SEFyfXR6kE04gGdCtXZN-So6nNJAAe1qwnTkl0UuoFpCEEuQhwkZND77o1Y9OVuVus8mgGtyAdTi-A7_0MkDKgBw"
                        }
                    },
                    "reponse": null,
                    "state": "Pending"
                }
            ]
        )),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_approvals_handler(
    node: Api,
    parameters: GetApprovalsQuery,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let status = match parameters.status {
        None => None,
        Some(value) => match value.to_lowercase().as_str() {
            "pending" => Some(ApprovalState::Pending),
            "obsolete" => Some(ApprovalState::Obsolete),
            "responded_accepted" => Some(ApprovalState::RespondedAccepted),
            "responded_rejected" => Some(ApprovalState::RespondedRejected),
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
                .map(ApprovalEntityResponse::from)
                .collect::<Vec<ApprovalEntityResponse>>()
        });
    handle_data(data)
}

/// Get approval by ID
///
/// Allows you to obtain a request for approval by its identifier.
#[utoipa::path(
    get,
    path = "/approval-requests/{id}",
    operation_id = "Get Approval Request Data",
    tag = "Approvals",
    context_path = "/api",
    params(
        ("id" = String, Path, description = "Approval's unique id")
    ),
    responses(
        (status = 200, description = "Approval Data successfully retrieved", body = ApprovalEntityResponse,
        example = json!(
            {
                "id": "J5dfpH-ahrqSo-od4jyZkubyO-XWFJSQ9maK73jKI4Ao",
                "request": {
                    "event_request": {
                        "Fact": {
                            "subject_id": "JoifaSpfenD2bEPeBLvUTWh30brm4tKcvdW8exQnkGoQ",
                            "payload": {
                                "Patch": {
                                    "data": [
                                        {
                                            "op": "add",
                                            "path": "/members/0",
                                            "value": {
                                                "id": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                                                "name": "WPO"
                                            }
                                        }
                                    ]
                                }
                            }
                        },
                        "signature": {
                            "signer": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                            "timestamp": 1688643580,
                            "value": "SE4yS1Q1Smhm3Az3r6WNFKAGd2Us69vyUA3j5q_riE6MICh_Ub2fSLxNS3Nn-g_CpppvABq6s_c8dF5kbmUir4Ag"
                        }
                    },
                    "sn": 1,
                    "gov_version": 0,
                    "patch": [
                        {
                            "op": "add",
                            "path": "/members/0",
                            "value": {
                                "id": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                                "name": "WPO"
                            }
                        }
                    ],
                    "state_hash": "JbDVCZxkDkZ5gLCc7Ge5X75pHHf8dA7_s8UynsnzG5o8",
                    "hash_prev_event": "JLic8SLrT7tJxA9B3aLaaKaIEuV7Wouo2ogHCid6O4g8",
                    "signature": {
                        "signer": "E0gaiDcPRVmYLUGbseHmBk2_2H-FAlSgaO6ZMOXhh4Gs",
                        "timestamp": 1688643580,
                        "value": "SEFyfXR6kE04gGdCtXZN-So6nNJAAe1qwnTkl0UuoFpCEEuQhwkZND77o1Y9OVuVus8mgGtyAdTi-A7_0MkDKgBw"
                    }
                },
                "reponse": null,
                "state": "Pending"
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_approval_handler(
    id: String,
    node: Api,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let result = if let Ok(id) = DigestIdentifier::from_str(&id) {
        node.get_approval(id)
            .await
            .map(ApprovalEntityResponse::from)
    } else {
        Err(ApiError::InvalidParameters(
            "ID specified is not a valid Digest Identifier".to_string(),
        ))
    };
    handle_data(result)
}

/// Emit approval for request
///
/// Allows you to issue an affirmative or negative approval for a previously received request.
#[utoipa::path(
    patch,
    path = "/approval-requests/{id}",
    operation_id = "Set your Aprroval for a request",
    tag = "Approvals",
    context_path = "/api",
    request_body(content = PatchVoteBody, content_type = "application/json", description = "Vote of the user for an existing request",
    example = json!(
        {
            "approvalType": "Accept"
        }
    )),
    params(
        ("id" = String, Path, description = "Approval's unique id"),
    ),
    responses(
        (status = 204, description = "Request successfully voted", body = ApprovalEntityResponse,
        example = json!(
            {
                "id": "J5dfpH-ahrqSo-od4jyZkubyO-XWFJSQ9maK73jKI4Ao",
                "request": {
                    "event_request": {
                        "Fact": {
                            "subject_id": "JoifaSpfenD2bEPeBLvUTWh30brm4tKcvdW8exQnkGoQ",
                            "payload": {
                                "Patch": {
                                    "data": [
                                        {
                                            "op": "add",
                                            "path": "/members/0",
                                            "value": {
                                                "id": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                                                "name": "WPO"
                                            }
                                        }
                                    ]
                                }
                            }
                        },
                        "signature": {
                            "signer": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                            "timestamp": 1688643580,
                            "value": "SE4yS1Q1Smhm3Az3r6WNFKAGd2Us69vyUA3j5q_riE6MICh_Ub2fSLxNS3Nn-g_CpppvABq6s_c8dF5kbmUir4Ag"
                        }
                    },
                    "sn": 1,
                    "gov_version": 0,
                    "patch": [
                        {
                            "op": "add",
                            "path": "/members/0",
                            "value": {
                                "id": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                                "name": "WPO"
                            }
                        }
                    ],
                    "state_hash": "JbDVCZxkDkZ5gLCc7Ge5X75pHHf8dA7_s8UynsnzG5o8",
                    "hash_prev_event": "JLic8SLrT7tJxA9B3aLaaKaIEuV7Wouo2ogHCid6O4g8",
                    "signature": {
                        "signer": "E0gaiDcPRVmYLUGbseHmBk2_2H-FAlSgaO6ZMOXhh4Gs",
                        "timestamp": 1688643580,
                        "value": "SEFyfXR6kE04gGdCtXZN-So6nNJAAe1qwnTkl0UuoFpCEEuQhwkZND77o1Y9OVuVus8mgGtyAdTi-A7_0MkDKgBw"
                    }
                },
                "reponse": {
                    "appr_req_hash": "J5dfpH-ahrqSo-od4jyZkubyO-XWFJSQ9maK73jKI4Ao",
                    "approved": true,
                    "signature": {
                        "signer": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                        "timestamp": 168864361,
                        "value": "SERUEr362pHPIcORhUnYPxnW1A_jW675_yphYIQIKaO6wytdh7xwwNTXHW6Q1fs9F6ag8VpTy2DM_5ppRT7irFDg"
                    }
                },
                "state": "Responded"
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 409, description = "Conflict"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn patch_approval_handler(
    request_id: String,
    node: Api,
    body: PatchVoteBody,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let acceptance = match body {
        PatchVoteBody::RespondedAccepted => true,
        PatchVoteBody::RespondedRejected => false,
    };
    let result = if let Ok(id) = DigestIdentifier::from_str(&request_id) {
        node.approval_request(id, acceptance)
            .await
            .map(ApprovalEntityResponse::from)
    } else {
        Err(ApiError::InvalidParameters(
            "ID specified is not a valid Digest Identifier".to_string(),
        ))
    };
    handle_data(result)
}

/// Get authorized subjects
///
/// Allows to obtain the list of subjects that have been pre-authorized by the node, as well as the identifiers of the nodes from which to obtain them.
#[utoipa::path(
    get,
    path = "/allowed-subjects",
    operation_id = "Get Allowed Subject Data",
    tag = "Others",
    context_path = "/api",
    params(
        ("from" = Option<String>, Query, description = "Id of initial subject"),
        ("quantity" = Option<isize>, Query, description = "Quantity of subjects requested")
    ),
    responses(
        (status = 200, description = "Subject Data successfully retrieved", body = [PreauthorizedSubjectsResponse],
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
    node: Api,
    parameters: GetWithPaginationString,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let result = node
        .get_all_allowed_subjects_and_providers(parameters.from, parameters.quantity)
        .await
        .map(|x| Vec::from_iter(x.into_iter().map(PreauthorizedSubjectsResponse::from)));
    handle_data(result)
}

/// Set subject as preauthorized
///
/// Allows a subject to be established as pre-qualified. It can also be used to specify from which nodes in the network the resource should be obtained.
#[utoipa::path(
    put,
    path = "/allowed-subjects/{id}",
    operation_id = "Put Allowed Subject Data",
    tag = "Others",
    context_path = "/api",
    params(
        ("id" = String, Path, description = "Subject's unique id")
    ),
    request_body(content = AuthorizeSubjectBody, content_type = "application/json", description = "Vote of the user for an existing request",
    example = json!(
        {
            "providers": []
        }
    )),
    responses(
        (status = 200, description = "Subject Data successfully created", body = AuthorizeSubjectBody,
        example = json!(
            {
                "providers": []
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn put_allowed_subjects_handler(
    id: String,
    node: Api,
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

/// Register KeyPair
///
/// It allows to generate a pair of cryptographic keys in the node that can then be assigned to a subject. The private key is never revealed.
#[utoipa::path(
    post,
    path = "/keys",
    tag = "Others",
    operation_id = "createKeys",
    context_path = "/api",
    params(
        ("algorithm" = String, Path, description = "Type of algorithm to use (possibilities: Ed25519, Secp256k1)")
    ),
    responses(
        (status = 201, description = "Public Key of the generated keypair", body = String,
        example = json!(
            {
                "public_key": "ELZ_b-kZzdPykcYuRNC2ZZe_2lCTCUoo60GXfR4cuXMw"
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn post_generate_keys_handler(
    node: Api,
    parameters: AddKeysQuery,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let derivator = parameters
        .algorithm
        .unwrap_or(KeyAlgorithms::Ed25519)
        .into();
    match node.add_keys(derivator).await {
        Ok(key) => handle_data(Ok(serde_json::json!({
            "public_key": key.to_str(),
        }))),
        Err(error) => handle_data(Err::<Value, ApiError>(error)),
    }
}

/// Send event request
///
/// Allows to send an event request for a subject to the TAPLE node.
/// These requests can be of any type of event (done, creation, transfer and end of life).
/// In case of external invocation, the requests can be signed.
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
            {
                "request_id": "J8618wGO7hH4wRuEeL0Ob5XNI9Q73BlCNlV8cWBORq78"
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 409, description = "Conflict"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn post_event_request_handler(
    node: Api,
    keys: KeyPair,
    derivator: KeyDerivator,
    digest_derivator: DigestDerivator,
    mut body: PostEventRequestBodyPreSignature,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    // If event request is a creation one and it does not specify a public_key, then a random one must be generated
    if let bodys::EventRequestBody::Create(creation_req) = &mut body.request {
        if creation_req.public_key.is_none() {
            let public_key = node.add_keys(derivator).await;
            if public_key.is_err() {
                return handle_data(public_key);
            }
            let public_key = public_key.unwrap().to_str();
            creation_req.public_key = Some(public_key);
        }
    }
    let Ok(request) = body.request.try_into() else {
        return Err(warp::reject::custom(
            Error::InvalidParameters{error: "Invalid request".to_owned()},
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
        None => Signature::new(&request, &keys, digest_derivator).expect("Error signing request"),
    };
    match node
        .external_request(Signed {
            content: request,
            signature,
        })
        .await
    {
        Ok(id) => handle_data(Ok(serde_json::json!({
            "request_id": id.to_str(),
        }))),
        Err(error) => handle_data(Err::<Value, ApiError>(error)),
    }
}

/// Get event request
///
/// Allows to obtain an event request by its identifier
#[utoipa::path(
    get,
    path = "/event-requests/{id}",
    operation_id = "Get Event Request Data",
    tag = "Requests",
    context_path = "/api",
    params(
        ("id" = String, Path, description = "Event Request's unique id"),
    ),
    responses(
        (status = 200, description = "Request Data successfully retrieved", body = TapleRequestResponse,
        example = json!(
            {
                "Fact": {
                    "subject_id": "JoifaSpfenD2bEPeBLvUTWh30brm4tKcvdW8exQnkGoQ",
                    "payload": {
                        "Patch": {
                            "data": [
                                {
                                    "op": "add",
                                    "path": "/members/0",
                                    "value": {
                                        "id": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                                        "name": "WPO"
                                    }
                                }
                            ]
                        }
                    }
                },
                "signature": {
                    "signer": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                    "timestamp": 1688643580,
                    "value": "SE4yS1Q1Smhm3Az3r6WNFKAGd2Us69vyUA3j5q_riE6MICh_Ub2fSLxNS3Nn-g_CpppvABq6s_c8dF5kbmUir4Ag"
                }
            }
        )
    ),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_taple_request_handler(
    request_id: String,
    node: Api,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let result = if let Ok(id) = DigestIdentifier::from_str(&request_id) {
        node.get_request(id).await.map(TapleRequestResponse::from)
    } else {
        Err(ApiError::InvalidParameters(
            "ID specified is not a valid Digest Identifier".to_string(),
        ))
    };
    handle_data(result)
}

/// Get event request state
///
/// Allows to obtain the status of an event request by its identifier.
#[utoipa::path(
    get,
    path = "/event-requests/{id}/state",
    operation_id = "Get Event Request State Data",
    tag = "Requests",
    context_path = "/api",
    params(
        ("id" = String, Path, description = "Event Request's unique id"),
    ),
    responses(
        (status = 200, description = "Request Data successfully retrieved", body = TapleRequestStateResponse,
        example = json!(
            {
                "id": "JyyWIjUa3Ui04oTSN4pJFT8FhmgPRPXzsG4_tIX8IBFg",
                "subject_id": "JoifaSpfenD2bEPeBLvUTWh30brm4tKcvdW8exQnkGoQ",
                "sn": 1,
                "state": "finished",
                "success": true
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_taple_request_state_handler(
    request_id: String,
    node: Api,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let result = if let Ok(id) = DigestIdentifier::from_str(&request_id) {
        node.get_request(id)
            .await
            .map(TapleRequestStateResponse::from)
    } else {
        Err(ApiError::InvalidParameters(
            "ID specified is not a valid Digest Identifier".to_string(),
        ))
    };
    handle_data(result)
}

/// Get subjects
///
/// Allows to obtain, with pagination, the list of subjects known by the node.
/// It can also be used to obtain exclusively the governances and all the subjects belonging to a specific one.
#[utoipa::path(
    get,
    path = "/subjects",
    tag = "Subjects",
    operation_id = "Get All Subjects Data",
    context_path = "/api",
    params(
        ("subject_type" = Option<String>, Query, description = "Type of subjects requested (possibilities: all, governances)"),
        ("governanceid" = Option<String>, Query, description = "Governance id of subjects requested"),
        ("from" = Option<String>, Query, description = "Identifier of the initial subject to be considered in pagination"),
        ("quantity" = Option<isize>, Query, description = "Quantity of subjects requested")
    ),
    responses(
        (status = 200, description = "Subjects Data successfully retrieved", body = [SubjectDataResponse],
        example = json!(
            [
                {
                    "subject_id": "JoifaSpfenD2bEPeBLvUTWh30brm4tKcvdW8exQnkGoQ",
                    "governance_id": "",
                    "sn": 0,
                    "public_key": "E0gaiDcPRVmYLUGbseHmBk2_2H-FAlSgaO6ZMOXhh4Gs",
                    "namespace": "",
                    "name": "Wine_Producers_Organization",
                    "schema_id": "governance",
                    "owner": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                    "creator": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                    "properties": {
                        "members": [],
                        "policies": [
                            {
                                "approve": {
                                    "quorum": "MAJORITY"
                                },
                                "evaluate": {
                                    "quorum": "MAJORITY"
                                },
                                "id": "governance",
                                "validate": {
                                    "quorum": "MAJORITY"
                                }
                            }
                        ],
                        "roles": [],
                        "schemas": []
                    },
                    "active": true
                }
            ]
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_subjects_handler(
    node: Api,
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
                if parameters.governanceid.is_some() {
                    return handle_data::<SubjectDataResponse>(Err(ApiError::InvalidParameters(
                        "governanceid can not be specified with subject_type=governances"
                            .to_string(),
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
            .map(SubjectDataResponse::from)
            .collect::<Vec<SubjectDataResponse>>()
    });
    handle_data(data)
}

/// Get subject by ID
/// Allows to obtain a specific subject by means of its identifier
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
                "subject_id": "JoifaSpfenD2bEPeBLvUTWh30brm4tKcvdW8exQnkGoQ",
                "governance_id": "",
                "sn": 0,
                "public_key": "E0gaiDcPRVmYLUGbseHmBk2_2H-FAlSgaO6ZMOXhh4Gs",
                "namespace": "",
                "name": "Wine_Producers_Organization",
                "schema_id": "governance",
                "owner": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                "creator": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                "properties": {
                    "members": [],
                    "policies": [
                        {
                            "approve": {
                                "quorum": "MAJORITY"
                            },
                            "evaluate": {
                                "quorum": "MAJORITY"
                            },
                            "id": "governance",
                            "validate": {
                                "quorum": "MAJORITY"
                            }
                        }
                    ],
                    "roles": [],
                    "schemas": []
                },
                "active": true
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_subject_handler(id: String, node: Api) -> Result<Box<dyn warp::Reply>, Rejection> {
    let response = if let Ok(id) = DigestIdentifier::from_str(&id) {
        node.get_subject(id).await.map(SubjectDataResponse::from)
    } else {
        Err(ApiError::InvalidParameters(
            "ID specified is not a valid Digest Identifier".to_string(),
        ))
    };
    handle_data(response)
}

/// Get validation proof
///
/// Allows to obtain the validation test of the last event for a specified subject.
#[utoipa::path(
    get,
    path = "/subjects/{id}/validation",
    operation_id = "getValidation",
    tag = "Subjects",
    context_path = "/api",
    params(
        ("id" = String, Path, description = "Subject's unique id"),
    ),
    responses(
        (status = 200, description = "Subjects Data successfully retrieved", body = GetProofResponse,
        example = json!(
            {
                "proof": {
                    "subject_id": "JoifaSpfenD2bEPeBLvUTWh30brm4tKcvdW8exQnkGoQ",
                    "schema_id": "governance",
                    "namespace": "",
                    "name": "Wine_Producers_Organization",
                    "subject_public_key": "E0gaiDcPRVmYLUGbseHmBk2_2H-FAlSgaO6ZMOXhh4Gs",
                    "governance_id": "",
                    "genesis_governance_version": 0,
                    "sn": 0,
                    "prev_event_hash": "",
                    "event_hash": "JLic8SLrT7tJxA9B3aLaaKaIEuV7Wouo2ogHCid6O4g8",
                    "governance_version": 0
                },
                "signatures": [
                    {
                        "signer": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                        "timestamp": 1688643031,
                        "value": "SEF3qN1uKIgNfnK6YlgU7rlCvDCNHhl_tdcRBvQRyGShR8oOOw5tVk8_OUNlyaJV_HsrISeX8jAf4L3diodRZ_Dg"
                    }
                ]
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_validation_proof_handle(
    id: String,
    node: Api,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let result = if let Ok(id) = DigestIdentifier::from_str(&id) {
        node.get_validation_proof(id)
            .await
            .map(|(signatures, proof)| GetProofResponse {
                proof: ValidationProofResponse::from(proof),
                signatures: signatures
                    .into_iter()
                    .map(SignatureBody::from)
                    .collect::<Vec<SignatureBody>>(),
            })
    } else {
        Err(ApiError::InvalidParameters(
            "ID specified is not a valid Digest Identifier".to_string(),
        ))
    };
    handle_data(result)
}

/// Get events of a subject
///
/// Allows to obtain, with pagination, the list of events of a subject.
#[utoipa::path(
    get,
    path = "/subjects/{id}/events",
    operation_id = "getEvents",
    context_path = "/api",
    tag = "Subjects",
    params(
        ("id" = String, Path, description = "Subject's unique id"),
        ("from" = Option<usize>, Query, description = "SN from which the event list should begin"),
        ("quantity" = Option<usize>, Query, description = "Quantity of events requested"),
    ),
    responses(
        (status = 200, description = "Subjects Data successfully retrieved", body = [SignedEvent],
        example = json!(
            [
                {
                    "subject_id": "JoifaSpfenD2bEPeBLvUTWh30brm4tKcvdW8exQnkGoQ",
                    "event_request": {
                        "Create": {
                            "governance_id": "",
                            "schema_id": "governance",
                            "namespace": "",
                            "name": "Wine_Producers_Organization",
                            "public_key": "E0gaiDcPRVmYLUGbseHmBk2_2H-FAlSgaO6ZMOXhh4Gs"
                        },
                        "signature": {
                            "signer": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                            "timestamp": 1688643031,
                            "value": "SE-tHjb3eWcMvVIYuSBPn0EW4Q5mQs2uswS5HLl0GB0iYVEc5jcOWD78ZHRL8VlO0mtxv9KWt2EI9R9Id2Z5o8CA"
                        }
                    },
                    "sn": 0,
                    "patch": [
                        {
                            "op": "add",
                            "path": "/members",
                            "value": []
                        },
                        {
                            "op": "add",
                            "path": "/policies",
                            "value": [
                                {
                                    "approve": {
                                        "quorum": "MAJORITY"
                                    },
                                    "evaluate": {
                                        "quorum": "MAJORITY"
                                    },
                                    "id": "governance",
                                    "validate": {
                                        "quorum": "MAJORITY"
                                    }
                                }
                            ]
                        },
                        {
                            "op": "add",
                            "path": "/roles",
                            "value": []
                        },
                        {
                            "op": "add",
                            "path": "/schemas",
                            "value": []
                        }
                    ],
                    "state_hash": "JVKr8BAEs1DhpNjMZf4525IYps2Gu6m7ZBmuaNBoM_Qk",
                    "eval_success": true,
                    "appr_required": false,
                    "approved": true,
                    "hash_prev_event": "",
                    "evaluators": [],
                    "approvers": [],
                    "signature": {
                        "signer": "E0gaiDcPRVmYLUGbseHmBk2_2H-FAlSgaO6ZMOXhh4Gs",
                        "timestamp": 168864303,
                        "value": "SEnTz4Nw-rX6y00yNF01o__AwyWxyG1s669AetXCfrnxCTSyf67xv8AsnccTOe4fFm-2ZIeRjubdf5FTQHZAd7BQ"
                    }
                }
            ]
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_events_of_subject_handler(
    id: String,
    node: Api,
    parameters: GetWithPagination,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let result = if let Ok(id) = DigestIdentifier::from_str(&id) {
        node.get_events(id, parameters.from, parameters.quantity)
            .await
            .map(|ve| {
                ve.into_iter()
                    .map(|e| SignedEvent(SignedBody::<EventContentResponse>::from(e)))
                    .collect::<Vec<SignedEvent>>()
            })
    } else {
        Err(ApiError::InvalidParameters(
            "ID specified is not a valid Digest Identifier".to_string(),
        ))
    };
    handle_data::<Vec<SignedEvent>>(result)
}

/// Get an event from a subject
///
/// Allows to obtain a specific event from a subject
#[utoipa::path(
    get,
    path = "/subjects/{id}/events/{sn}",
    operation_id = "getEvent",
    tag = "Subjects",
    context_path = "/api",
    params(
        ("id" = String, Path, description = "Subject's unique id"),
        ("sn" = u64, Path, description = "Event sn"),
    ),
    responses(
        (status = 200, description = "Subjects Data successfully retrieved", body = SignedEvent,
        example = json!(
            {
                "subject_id": "JoifaSpfenD2bEPeBLvUTWh30brm4tKcvdW8exQnkGoQ",
                "event_request": {
                    "Create": {
                        "governance_id": "",
                        "schema_id": "governance",
                        "namespace": "",
                        "name": "Wine_Producers_Organization",
                        "public_key": "E0gaiDcPRVmYLUGbseHmBk2_2H-FAlSgaO6ZMOXhh4Gs"
                    },
                    "signature": {
                        "signer": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                        "timestamp": 168864303,
                        "value": "SE-tHjb3eWcMvVIYuSBPn0EW4Q5mQs2uswS5HLl0GB0iYVEc5jcOWD78ZHRL8VlO0mtxv9KWt2EI9R9Id2Z5o8CA"
                    }
                },
                "sn": 0,
                "gov_version": 0,
                "patch": [
                    {
                        "op": "add",
                        "path": "/members",
                        "value": []
                    },
                    {
                        "op": "add",
                        "path": "/policies",
                        "value": [
                            {
                                "approve": {
                                    "quorum": "MAJORITY"
                                },
                                "evaluate": {
                                    "quorum": "MAJORITY"
                                },
                                "id": "governance",
                                "validate": {
                                    "quorum": "MAJORITY"
                                }
                            }
                        ]
                    },
                    {
                        "op": "add",
                        "path": "/roles",
                        "value": []
                    },
                    {
                        "op": "add",
                        "path": "/schemas",
                        "value": []
                    }
                ],
                "state_hash": "JVKr8BAEs1DhpNjMZf4525IYps2Gu6m7ZBmuaNBoM_Qk",
                "eval_success": true,
                "appr_required": false,
                "approved": true,
                "hash_prev_event": "",
                "evaluators": [],
                "approvers": [],
                "signature": {
                    "signer": "E0gaiDcPRVmYLUGbseHmBk2_2H-FAlSgaO6ZMOXhh4Gs",
                    "timestamp": 168864303,
                    "value": "SEnTz4Nw-rX6y00yNF01o__AwyWxyG1s669AetXCfrnxCTSyf67xv8AsnccTOe4fFm-2ZIeRjubdf5FTQHZAd7BQ"
                }
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_event_handler(
    id: String,
    sn: u64,
    node: Api,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let response = if let Ok(id) = DigestIdentifier::from_str(&id) {
        node.get_event(id, sn).await.map(|e| SignedEvent(e.into()))
    } else {
        Err(ApiError::InvalidParameters(
            "ID specified is not a valid Digest Identifier".to_string(),
        ))
    };
    handle_data(response)
}

pub fn handle_data<T: Serialize + std::fmt::Debug>(
    data: Result<T, ApiError>,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    match &data {
        Ok(data) => Ok(Box::new(warp::reply::json(&data))),
        Err(ApiError::InvalidParameters(msg)) => {
            Err(warp::reject::custom(Error::InvalidParameters {
                error: msg.to_owned(),
            }))
        }
        Err(ApiError::Conflict(msg)) => Err(warp::reject::custom(Error::Conflict {
            error: msg.to_owned(),
        })),
        Err(ApiError::NotFound(msg)) => Err(warp::reject::custom(Error::NotFound {
            error: msg.to_owned(),
        })),
        Err(ApiError::EventCreationError { .. }) => {
            Err(warp::reject::custom(Error::ExecutionError {
                source: data.unwrap_err(),
            }))
        }
        Err(ApiError::NotEnoughPermissions(msg)) => {
            Err(warp::reject::custom(Error::NotEnoughPermissions {
                error: msg.to_owned(),
            }))
        }
        // Err(ApiError::VoteNotNeeded(msg)) => Err(warp::reject::custom(Error::RequestError(msg.to_owned()))),
        Err(error) => Err(warp::reject::custom(Error::ExecutionError {
            source: error.to_owned(),
        })),
    }
}
