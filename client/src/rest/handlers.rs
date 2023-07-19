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
        TapleRequestStateResponse, ValidationProofResponse, SignedEvent,
    },
};

#[utoipa::path(
    get,
    path = "/approval-requests",
    operation_id = "Get Approval Request Data",
    tag = "Aproval",
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
    path = "/approval-requests/{id}",
    operation_id = "Get Approval Request Data",
    tag = "Aproval",
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
    patch,
    path = "/approval-requests/{id}",
    operation_id = "Set your Aprroval for a request",
    tag = "Aproval",
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
    path = "/allowed-subjects",
    operation_id = "Get Allowed Subject Data",
    tag = "",
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
    tag = "",
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
    post,
    path = "/keys",
    tag = "",
    operation_id = "createKeys",
    context_path = "/api",
    params(
        ("algorithm" = String, Path, description = "Type of algorithm to use (possibilities: Ed25519, Secp256k1)")
    ),
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
                "request": {
                    "Fact": {
                        "subject_id": "JoifaSpfenD2bEPeBLvUTWh30brm4tKcvdW8exQnkGoQ",
                        "payload": {
                            "Patch": {
                                "data": [{
                                    "op": "add",
                                    "path": "/members/0",
                                    "value": {
                                    "id": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                                    "name": "WPO"
                                    }
                                }]
                            }
                        }
                    }
                }
            }
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
    get,
    path = "/subjects",
    tag = "Subjects",
    operation_id = "Get All Subjects Data",
    context_path = "/api",
    params(
        ("subject_type" = Option<String>, Query, description = "Type of subjects requested (possibilities: all, governances)"),
        ("governanceid" = Option<String>, Query, description = "Governance id of subjects requested"),
        ("from" = Option<String>, Query, description = "Id of initial subject"),
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

#[utoipa::path(
    get,
    path = "/subjects/{id}/events",
    operation_id = "getEvents",
    context_path = "/api",
    tag = "Subjects",
    params(
        ("id" = String, Path, description = "Subject's unique id"),
        ("from" = Option<usize>, Query, description = "Initial SN"),
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
    node: NodeAPI,
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
        Err(ApiError::InvalidParameters(format!(
            "ID specified is not a valid Digest Identifier"
        )))
    };
    handle_data::<Vec<SignedEvent>>(result)
}

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
    node: NodeAPI,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let response = if let Ok(id) = DigestIdentifier::from_str(&id) {
        node.get_event(id, sn).await.map(|e| SignedEvent(e.into()))
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