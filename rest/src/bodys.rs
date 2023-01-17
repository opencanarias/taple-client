use core::{
    event_request::RequestPayload, ApiError, CreateRequest, CreateType, ExternalEventRequest,
    SignatureRequest, StateType,
};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Eq, Deserialize, ToSchema)]
pub enum Payload {
    #[schema(value_type = Object)]
    Json(serde_json::Value),
    #[schema(value_type = Object)]
    JsonPatch(serde_json::Value),
}

impl Into<RequestPayload> for Payload {
    fn into(self) -> RequestPayload {
        match self {
            Self::Json(data) => RequestPayload::Json(serde_json::to_string(&data).unwrap()),
            Self::JsonPatch(data) => {
                RequestPayload::JsonPatch(serde_json::to_string(&data).unwrap())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PostSubjectBody {
    pub governance_id: String,
    pub schema_id: String,
    pub namespace: String,
    pub payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostEventRequestBody {
    pub request: EventRequestTypeBody,
    pub timestamp: Option<i64>,
    pub signature: Option<SignatureRequest>,
}

impl TryInto<ExternalEventRequest> for PostEventRequestBody {
    type Error = ApiError;
    fn try_into(self) -> Result<ExternalEventRequest, Self::Error> {
        let EventRequestTypeBody::State(request) = self.request else {
            return Err(ApiError::InvalidParameters);
        };
        let Some(timestamp) = self.timestamp else {
            return Err(ApiError::InvalidParameters);
        };
        let Some(signature) = self.signature else {
            return Err(ApiError::InvalidParameters);
        };
        Ok(ExternalEventRequest {
            request: StateType {
                subject_id: request.subject_id,
                payload: request.payload.into(),
            },
            timestamp,
            signature,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum EventRequestTypeBody {
    Create(CreateRequestBody),
    State(StateRequestBody),
}

impl Into<CreateRequest> for EventRequestTypeBody {
    fn into(self) -> CreateRequest {
        match self {
            Self::Create(data) => CreateRequest::Create(CreateType {
                governance_id: data.governance_id,
                schema_id: data.schema_id,
                namespace: data.namespace,
                payload: data.payload.into(),
            }),
            Self::State(data) => CreateRequest::State(StateType {
                subject_id: data.subject_id,
                payload: data.payload.into(),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct CreateRequestBody {
    pub governance_id: String,
    pub schema_id: String,
    pub namespace: String,
    pub payload: Payload,
}

#[derive(Debug, Clone, PartialEq, Serialize, Eq, Deserialize, ToSchema)]
pub struct StateRequestBody {
    pub subject_id: String,
    pub payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostEventBody {
    pub subject_id: String,
    pub payload: Payload,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PostGovernanceBody {
    pub payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SignatureRequestContent {
    pub signer: String,             // KeyIdentifier,
    pub event_content_hash: String, // DigestIdentifier,
    pub timestamp: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "approvalType")]
pub enum PutVoteBody {
    Accept,
    Reject,
}
