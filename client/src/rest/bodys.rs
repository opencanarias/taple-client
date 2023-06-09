use std::{str::FromStr};

use serde::{Deserialize, Serialize};
use taple_core::{
    event_request::{
        CreateRequest, EOLRequest, EventRequest, EventRequestType, StateRequest, TransferRequest,
    },
    identifier::{Derivable, DigestIdentifier, KeyIdentifier, SignatureIdentifier},
    signature::{Signature, SignatureContent},
    ApiError, TimeStamp,
};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum EventRequestTypeBody {
    Create(CreateRequestBody),
    State(StateRequestBody),
    Transfer(TransferRequestBody),
    EOL(EOLRequestBody),
}

impl TryFrom<EventRequestType> for EventRequestTypeBody {
    type Error = ApiError;
    fn try_from(value: EventRequestType) -> Result<Self, Self::Error> {
        match value {
            EventRequestType::Create(data) => Ok(EventRequestTypeBody::Create(data.try_into()?)),
            EventRequestType::State(data) => Ok(EventRequestTypeBody::State(data.try_into()?)),
            EventRequestType::Transfer(data) => {
                Ok(EventRequestTypeBody::Transfer(data.try_into()?))
            }
            EventRequestType::EOL(data) => Ok(EventRequestTypeBody::EOL(data.try_into()?)),
        }
    }
}

impl TryInto<EventRequestType> for EventRequestTypeBody {
    type Error = ApiError;
    fn try_into(self) -> Result<EventRequestType, Self::Error> {
        match self {
            EventRequestTypeBody::Create(data) => Ok(EventRequestType::Create(data.try_into()?)),
            EventRequestTypeBody::State(data) => Ok(EventRequestType::State(data.try_into()?)),
            EventRequestTypeBody::Transfer(data) => {
                Ok(EventRequestType::Transfer(data.try_into()?))
            }
            EventRequestTypeBody::EOL(data) => Ok(EventRequestType::EOL(data.try_into()?)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRequestBody {
    pub governance_id: String,
    pub schema_id: String,
    pub namespace: String,
}

impl TryFrom<CreateRequest> for CreateRequestBody {
    type Error = ApiError;
    fn try_from(value: CreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            governance_id: value.governance_id.to_str(),
            schema_id: value.schema_id,
            namespace: value.namespace,
        })
    }
}

impl TryInto<CreateRequest> for CreateRequestBody {
    type Error = ApiError;

    fn try_into(self) -> Result<CreateRequest, Self::Error> {
        Ok(CreateRequest {
            governance_id: DigestIdentifier::from_str(&self.governance_id).map_err(|_| {
                ApiError::InvalidParameters(format!("Invalid DigestIdentifier for governance id"))
            })?,
            schema_id: self.schema_id,
            namespace: self.namespace,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EOLRequestBody {
    pub subject_id: String,
}

impl TryInto<EOLRequest> for EOLRequestBody {
    type Error = ApiError;

    fn try_into(self) -> Result<EOLRequest, Self::Error> {
        Ok(EOLRequest {
            subject_id: DigestIdentifier::from_str(&self.subject_id).map_err(|_| {
                ApiError::InvalidParameters(format!("Invalid DigestIdentifier for subject id"))
            })?,
        })
    }
}

impl TryFrom<EOLRequest> for EOLRequestBody {
    type Error = ApiError;
    fn try_from(value: EOLRequest) -> Result<Self, Self::Error> {
        Ok(EOLRequestBody {
            subject_id: value.subject_id.to_str(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TransferRequestBody {
    pub subject_id: String,
    pub public_key: String,
}

impl TryInto<TransferRequest> for TransferRequestBody {
    type Error = ApiError;

    fn try_into(self) -> Result<TransferRequest, Self::Error> {
        Ok(TransferRequest {
            subject_id: DigestIdentifier::from_str(&self.subject_id).map_err(|_| {
                ApiError::InvalidParameters(format!("Invalid DigestIdentifier for subject id"))
            })?,
            public_key: KeyIdentifier::from_str(&self.public_key).map_err(|_| {
                ApiError::InvalidParameters(format!("Invalid KeyIdentifier for public key"))
            })?,
        })
    }
}

impl TryFrom<TransferRequest> for TransferRequestBody {
    type Error = ApiError;
    fn try_from(value: TransferRequest) -> Result<Self, Self::Error> {
        Ok(TransferRequestBody {
            subject_id: value.subject_id.to_str(),
            public_key: value.public_key.to_str(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StateRequestBody {
    pub subject_id: String,
    pub invokation: String,
}

impl TryFrom<StateRequest> for StateRequestBody {
    type Error = ApiError;
    fn try_from(value: StateRequest) -> Result<Self, Self::Error> {
        Ok(StateRequestBody {
            subject_id: value.subject_id.to_str(),
            invokation: value.invokation,
        })
    }
}

impl TryInto<StateRequest> for StateRequestBody {
    type Error = ApiError;
    fn try_into(self) -> Result<StateRequest, Self::Error> {
        Ok(StateRequest {
            subject_id: DigestIdentifier::from_str(&self.subject_id).map_err(|_| {
                ApiError::InvalidParameters(format!("Invalid DigestIdentifier for subject id"))
            })?,
            invokation: self.invokation,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExpectingTransfer {
    pub subject_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthorizeSubjectBody {
    pub subject_id: String,
    pub providers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostEventRequestBody {
    pub request: EventRequestTypeBody,
    pub timestamp: u64,
    pub signature: SignatureRequest,
}

impl TryFrom<EventRequest> for PostEventRequestBody {
    type Error = ApiError;
    fn try_from(value: EventRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            request: value.request.try_into()?,
            timestamp: value.timestamp.time,
            signature: value.signature.try_into()?,
        })
    }
}

impl TryInto<EventRequest> for PostEventRequestBody {
    type Error = ApiError;
    fn try_into(self) -> Result<EventRequest, Self::Error> {
        Ok(EventRequest {
            request: self.request.try_into()?,
            timestamp: TimeStamp {
                time: self.timestamp,
            },
            signature: self.signature.try_into()?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SignatureRequest {
    pub content: SignatureRequestContent,
    pub signature: String, // SignatureIdentifier,
}

impl TryFrom<Signature> for SignatureRequest {
    type Error = ApiError;
    fn try_from(value: Signature) -> Result<Self, Self::Error> {
        Ok(Self {
            content: value.content.try_into()?,
            signature: value.signature.to_str(),
        })
    }
}

impl TryInto<Signature> for SignatureRequest {
    type Error = ApiError;

    fn try_into(self) -> Result<Signature, Self::Error> {
        Ok(Signature {
            content: self.content.try_into()?,
            signature: SignatureIdentifier::from_str(&self.signature).map_err(|_| {
                ApiError::InvalidParameters(format!("Invalid SignatureIdentifier for signature"))
            })?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SignatureRequestContent {
    pub signer: String,             // KeyIdentifier,
    pub event_content_hash: String, // DigestIdentifier,
    pub timestamp: u64,
}

impl TryFrom<SignatureContent> for SignatureRequestContent {
    type Error = ApiError;
    fn try_from(value: SignatureContent) -> Result<Self, Self::Error> {
        Ok(Self {
            signer: value.signer.to_str(),
            event_content_hash: value.event_content_hash.to_str(),
            timestamp: value.timestamp.time,
        })
    }
}

impl TryInto<SignatureContent> for SignatureRequestContent {
    type Error = ApiError;

    fn try_into(self) -> Result<SignatureContent, Self::Error> {
        Ok(SignatureContent {
            signer: KeyIdentifier::from_str(&self.signer).map_err(|_| {
                ApiError::InvalidParameters(format!("Invalid KeyIdentifier for signer"))
            })?,
            event_content_hash: DigestIdentifier::from_str(&self.event_content_hash).map_err(
                |_| {
                    ApiError::InvalidParameters(format!("Invalid DigestIdentieir for content hash"))
                },
            )?,
            timestamp: TimeStamp {
                time: self.timestamp,
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "approvalType")]
pub enum PutVoteBody {
    Accept,
    Reject,
}
