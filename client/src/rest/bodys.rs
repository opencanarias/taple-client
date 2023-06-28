use std::str::FromStr;

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use taple_core::{
    identifier::{Derivable, DigestIdentifier, KeyIdentifier, SignatureIdentifier},
    request::{EOLRequest, EventRequest, FactRequest, StartRequest, TransferRequest},
    signature::{Signature, Signed},
    ApiError, TimeStamp, ValueWrapper,
};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct SignedBody<T: ToSchema + Clone + Debug> {
    #[serde(flatten)]
    pub content: T,
    pub signature: SignatureBody,
}

impl<C: BorshSerialize + BorshDeserialize, T: Clone + Debug + ToSchema + From<C>> From<Signed<C>>
    for SignedBody<T>
{
    fn from(value: Signed<C>) -> Self {
        Self {
            content: T::from(value.content),
            signature: SignatureBody::from(value.signature),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum EventRequestBody {
    Create(CreateRequestBody),
    Fact(FactRequestBody),
    Transfer(TransferRequestBody),
    EOL(EOLRequestBody),
}

impl From<EventRequest> for EventRequestBody {
    fn from(value: EventRequest) -> Self {
        match value {
            EventRequest::Create(data) => EventRequestBody::Create(data.into()),
            EventRequest::Fact(data) => EventRequestBody::Fact(data.into()),
            EventRequest::Transfer(data) => EventRequestBody::Transfer(data.into()),
            EventRequest::EOL(data) => EventRequestBody::EOL(data.into()),
        }
    }
}

impl TryInto<EventRequest> for EventRequestBody {
    type Error = ApiError;
    fn try_into(self) -> Result<EventRequest, Self::Error> {
        match self {
            EventRequestBody::Create(data) => Ok(EventRequest::Create(data.try_into()?)),
            EventRequestBody::Fact(data) => Ok(EventRequest::Fact(data.try_into()?)),
            EventRequestBody::Transfer(data) => Ok(EventRequest::Transfer(data.try_into()?)),
            EventRequestBody::EOL(data) => Ok(EventRequest::EOL(data.try_into()?)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRequestBody {
    pub governance_id: String,
    pub schema_id: String,
    pub namespace: String,
    pub name: String,
    pub public_key: Option<String>,
}

impl From<StartRequest> for CreateRequestBody {
    fn from(value: StartRequest) -> Self {
        Self {
            governance_id: value.governance_id.to_str(),
            schema_id: value.schema_id,
            namespace: value.namespace,
            name: value.name,
            public_key: Some(value.public_key.to_str()),
        }
    }
}

impl TryInto<StartRequest> for CreateRequestBody {
    type Error = ApiError;

    fn try_into(self) -> Result<StartRequest, Self::Error> {
        Ok(StartRequest {
            governance_id: DigestIdentifier::from_str(&self.governance_id).map_err(|_| {
                ApiError::InvalidParameters(format!("Invalid DigestIdentifier for governance id"))
            })?,
            schema_id: self.schema_id,
            namespace: self.namespace,
            name: self.name,
            public_key: KeyIdentifier::from_str(&self.public_key.unwrap()).map_err(|_| {
                ApiError::InvalidParameters(format!("Invalid DigestIdentifier for governance id"))
            })?,
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

impl From<EOLRequest> for EOLRequestBody {
    fn from(value: EOLRequest) -> Self {
        EOLRequestBody {
            subject_id: value.subject_id.to_str(),
        }
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

impl From<TransferRequest> for TransferRequestBody {
    fn from(value: TransferRequest) -> Self {
        TransferRequestBody {
            subject_id: value.subject_id.to_str(),
            public_key: value.public_key.to_str(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FactRequestBody {
    pub subject_id: String,
    pub payload: ValueWrapper,
}

impl From<FactRequest> for FactRequestBody {
    fn from(value: FactRequest) -> Self {
        FactRequestBody {
            subject_id: value.subject_id.to_str(),
            payload: value.payload,
        }
    }
}

impl TryInto<FactRequest> for FactRequestBody {
    type Error = ApiError;
    fn try_into(self) -> Result<FactRequest, Self::Error> {
        Ok(FactRequest {
            subject_id: DigestIdentifier::from_str(&self.subject_id).map_err(|_| {
                ApiError::InvalidParameters(format!("Invalid DigestIdentifier for subject id"))
            })?,
            payload: self.payload,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthorizeSubjectBody {
    pub providers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostEventRequestBodyPreSignature {
    pub request: EventRequestBody,
    pub signature: Option<SignatureBody>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SignatureBody {
    signer: String, // KeyIdentifier
    timestamp: u64,
    value: String, // SignatureIdentifier,
}

impl From<Signature> for SignatureBody {
    fn from(value: Signature) -> Self {
        Self {
            signer: value.signer.to_str(),
            timestamp: value.timestamp.0,
            value: value.value.to_str(),
        }
    }
}

impl TryInto<Signature> for SignatureBody {
    type Error = ApiError;

    fn try_into(self) -> Result<Signature, Self::Error> {
        Ok(Signature {
            signer: KeyIdentifier::from_str(&self.signer).map_err(|_| {
                ApiError::InvalidParameters(format!("Invalid KeyIdentifier for signature signer"))
            })?,
            timestamp: TimeStamp(self.timestamp),
            value: SignatureIdentifier::from_str(&self.value).map_err(|_| {
                ApiError::InvalidParameters(format!("Invalid SignatureIdentifier for signature"))
            })?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "approvalType")]
pub enum PatchVoteBody {
    Accept,
    Reject,
}
