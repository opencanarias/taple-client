use commons::models::{approval_signature::Acceptance, event::Event};
use serde::Serialize;
use warp::Rejection;

use crate::rest::bodys::PostEventRequestBody;
use taple_core::{ApiError, ApiModuleInterface, NodeAPI};

use super::{
    bodys::{PostEventBody, PutVoteBody},
    error::Error,
    querys::{GetAllSubjectsQuery, GetEventsQuery},
};

#[utoipa::path(
    get,
    path = "/subjects/{id}",
    operation_id = "Get Subject Data",
    tag = "Subjects",
    context_path = "/api",
    security(("api_key" = [])),
    params(
        ("id" = String, Path, description = "Subject's unique id")
    ),
    responses(
        (status = 200, description = "Subject Data successfully retrieved", body = SubjectData,
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
    _header: String,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    if id.is_empty() {
        return Err(warp::reject::custom(Error::InvalidParameters(
            "Error in query parameter".to_owned(),
        )));
    }
    let response = node.get_subject(id).await;
    handle_data(response)
}

#[utoipa::path(
    get,
    path = "/subjects",
    tag = "Subjects",
    operation_id = "Get All Subjects Data",
    context_path = "/api",
    security(("api_key" = [])),
    params(
        ("from" = Option<usize>, Query, description = "Number of initial subject"),
        ("quantity" = Option<usize>, Query, description = "Quantity of subjects requested")
    ),
    responses(
        (status = 200, description = "Subjects Data successfully retrieved", body = [SubjectData],
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
pub async fn get_all_subjects_handler(
    node: NodeAPI,
    _header: String,
    parameters: GetAllSubjectsQuery,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let data = node
        .get_all_subjects("namespace1".into(), parameters.from, parameters.quantity)
        .await;
    handle_data(data)
}

#[utoipa::path(
    post,
    path = "/requests",
    tag = "Requests",
    operation_id = "Create a new Event Request",
    context_path = "/api",
    security(("api_key" = [])),
    request_body(content = PostEventRequestBody, content_type = "application/json", description = "Event Request type and payload with the associated signature"),
    responses(
        (status = 202, description = "Event Request Created", body = RequestData,
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
    _header: String,
    node: NodeAPI,
    body: PostEventRequestBody,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let data;
    if body.signature.is_none() && body.timestamp.is_none() {
        data = node.create_request(body.request.into()).await;
    } else if body.signature.is_some() && body.timestamp.is_some() {
        match body.try_into() {
            Ok(external_request) => {
                data = node.external_request(external_request).await;
            }
            Err(error) => data = Err(error),
        }
    } else {
        return Err(warp::reject::custom(Error::BadRequest(
            "Must provide signature and timestamp in case of exernal request".into(),
        )));
    }
    handle_data(data)
}

#[utoipa::path(
    get,
    path = "/approvals",
    tag = "Approvals",
    operation_id = "Get all the pending requests for Approval",
    context_path = "/api",
    security(("api_key" = [])),
    responses(
        (status = 200, description = "All pending requests", body =  [EventRequest],
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
    _header: String,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let data = node.get_pending_requests().await;
    handle_data(data)
}

#[utoipa::path(
    get,
    path = "/approvals/{id}",
    tag = "Approvals",
    operation_id = "Get a specific pending request for Approval",
    context_path = "/api",
    security(("api_key" = [])),
    responses(
        (status = 200, description = "The pending request", body = EventRequest,
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
    _header: String,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let data = node.get_single_request(id).await;
    handle_data(data)
}

#[utoipa::path(
    put,
    path = "/approvals{id}",
    operation_id = "Set your Aprroval for a request",
    tag = "Approvals",
    context_path = "/api",
    security(("api_key" = [])),
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
    _header: String,
    node: NodeAPI,
    body: PutVoteBody,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let acceptance = match body {
        PutVoteBody::Accept => Acceptance::Accept,
        PutVoteBody::Reject => Acceptance::Reject,
    };
    let data = node.approval_request(request_id, acceptance).await;
    handle_data(data)
}
#[utoipa::path(
    get,
    path = "/governances/{id}",
    operation_id = "Get Governance Data",
    tag = "Governances",
    context_path = "/api",
    security(("api_key" = [])),
    params(
        ("id" = String, Path, description = "Governance's unique id")
    ),
    responses(
        (status = 200, description = "Subject Data successfully retrieved", body = SubjectData, 
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
    _header: String,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    if id.is_empty() {
        return Err(warp::reject::custom(Error::InvalidParameters(
            "Error in query parameter".to_owned(),
        )));
    }
    let mut response = node.get_subject(id).await;
    if response.is_ok() && !response.as_ref().unwrap().governance_id.digest.is_empty() {
        response = Err(ApiError::NotFound(String::from(
            "This ID does not belong to a governance",
        )));
    }
    handle_data(response)
}

#[utoipa::path(
    get,
    path = "/governances",
    operation_id = "Get all Governances data",
    tag = "Governances",
    context_path = "/api",
    security(("api_key" = [])),
    responses(
        (status = 200, description = "Subjets Data successfully retrieved", body = [SubjectData],
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
    _header: String,
    node: NodeAPI,
    parameters: GetAllSubjectsQuery,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let data = node
        .get_all_governances("namespace1".into(), parameters.from, parameters.quantity)
        .await;
    handle_data(data)
}

#[utoipa::path(
    get,
    path = "/subjects/{id}/events",
    operation_id = "Get all Events from indicated Subject",
    context_path = "/api",
    tag = "Events",
    security(("api_key" = [])),
    params(
        ("id" = String, Path, description = "Subject's unique id"),
        ("from" = Option<usize>, Query, description = "Initial SN"),
        ("quantity" = Option<usize>, Query, description = "Quantity of events requested"),
    ),
    responses(
        (status = 200, description = "Subjects Data successfully retrieved", body = [Event],
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
    _header: String,
    parameters: GetEventsQuery,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    if id.is_empty() {
        return Err(warp::reject::custom(Error::InvalidParameters(
            "Error in query parameter".to_owned(),
        )));
    }
    let data = node
        .get_event_of_subject(id, parameters.from, parameters.quantity)
        .await;
    handle_data::<Vec<Event>>(data)
}

#[utoipa::path(
    post,
    path = "/subjects/{id}/events/simulated",
    operation_id = "Simulate the creationg of an Event and get simulated Subject data",
    tag = "Events",
    security(("api_key" = [])),
    context_path = "/api",
    params(
        ("id" = String, Path, description = "Subject's unique id"),
    ),
    request_body(content = PostEventBody, content_type = "application/json", description = "SubjectID and payload of the event"),
    responses(
        (status = 202, description = "Event Simulated", body = SubjectData,
        example = json!(
            {
                "subject_id": "JKZgYhPjQdWNWWwkac0wSwqLKoOJsT0QimJmj6zjimWc",
                "governance_id": "J7BgD3dqZ8vO4WEH7-rpWIH-IhMqaSDnuJ3Jb8K6KvL0",
                "sn": 1,
                "public_key": "ELZ_b-kZzdPykcYuRNC2ZZe_2lCTCUoo60GXfR4cuXMw",
                "namespace": "namespace1",
                "schema_id": "Prueba",
                "owner": "EFXv0jBIr6BtoqFMR7G_JBSuozRc2jZnu5VGUH2gy6-w",
                "properties": "{\"localizacion\":\"Argentina\",\"temperatura\":-3}"
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not Found"),
        (status = 409, description = "Conflict"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn post_event_simulated_handler(
    id: String,
    node: NodeAPI,
    _header: String,
    body: PostEventBody,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    if id.is_empty() {
        return Err(warp::reject::custom(Error::InvalidParameters(
            "Error in query parameter".to_owned(),
        )));
    }
    let payload = body.payload.into();
    let data = node.simulate_event(id, payload).await;
    handle_data(data)
}

#[utoipa::path(
    get,
    path = "/subjects/{id}/events/{sn}",
    operation_id = "Get the Event data of indicated Events",
    tag = "Events",
    security(("api_key" = [])),
    context_path = "/api",
    params(
        ("id" = String, Path, description = "Subject's unique id"),
        ("sn" = u64, Path, description = "Event sn"),
    ),
    responses(
        (status = 200, description = "Subjects Data successfully retrieved", body = Event,
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
    _header: String,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    // TODO: Analyze if an alternative method is necessary
    if id.is_empty() {
        return Err(warp::reject::custom(Error::InvalidParameters(
            "Error in query parameter".to_owned(),
        )));
    }
    let response = node
        .get_event_of_subject(id, Some(sn as i64), Some(1))
        .await;
    if response.is_ok() {
        let Some(event) = response.unwrap().pop() else {
            return Err(warp::reject::custom(Error::NotFound("Event not found".into())));
        };
        handle_data::<Event>(Ok(event))
    } else {
        handle_data::<Vec<Event>>(response)
    }
}

#[utoipa::path(
    get,
    path = "/subjects/{id}/events/{sn}/properties",
    operation_id = "Get Event Properties",
    tag = "Events",
    security(("api_key" = [])),
    context_path = "/api",
    params(
        ("id" = String, Path, description = "Subject's unique id"),
        ("sn" = u64, Path, description = "Event sn"),
    ),
    responses(
        (status = 200, description = "Subjects Data successfully retrieved", body = EventRequestType,
        example = json!(
            {
                "State": {
                    "subject_id": "JKZgYhPjQdWNWWwkac0wSwqLKoOJsT0QimJmj6zjimWc",
                    "payload": {
                        "Json": "{\"localizacion\":\"Argentina\",\"temperatura\":-3}"
                    }
                }
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_event_properties_handler(
    id: String,
    sn: u64,
    node: NodeAPI,
    _header: String,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    if id.is_empty() {
        return Err(warp::reject::custom(Error::InvalidParameters(
            "Error in query parameter".to_owned(),
        )));
    }
    let data = node
        .get_event_of_subject(id, Some(sn as i64), Some(1))
        .await;
    if data.is_ok() {
        // TODO: Determine if it is possible to receive an empty array
        let event = data.unwrap().pop().unwrap();
        let properties = event.event_content.event_request.request;
        return Ok(Box::new(warp::reply::json(&properties)));
    } else {
        Err(warp::reject::custom(Error::ExecutionError {
            source: data.unwrap_err(),
        }))
    }
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
