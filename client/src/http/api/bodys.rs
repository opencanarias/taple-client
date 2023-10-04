use std::str::FromStr;

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use taple_core::{
    identifier::{Derivable, DigestIdentifier, KeyIdentifier, SignatureIdentifier},
    request::{EOLRequest, EventRequest, FactRequest, StartRequest, TransferRequest},
    signature::{Signature, Signed},
    ApiError, TimeStamp, ValueWrapper,
};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct SignedBody<T>
where
    T: for<'a> ToSchema<'a> + Clone + Debug,
{
    #[serde(flatten)]
    pub content: T,
    pub signature: SignatureBody,
}

impl<
        C: BorshSerialize + BorshDeserialize + Clone,
        T: for<'a> ToSchema<'a> + Clone + Debug + From<C>,
    > From<Signed<C>> for SignedBody<T>
{
    fn from(value: Signed<C>) -> Self {
        Self {
            content: T::from(value.content),
            signature: SignatureBody::from(value.signature),
        }
    }
}

// Utoipa does not properly process structures that accept generics.
// Consequently, we use this Wrapper pattern to be able to represent the structures
// in the resulting OpenAPI without also having to write entire new structures.
// Note that although the "ToSchema" trait is implemented manually, the implementation
// is simple by calling the internal representations of each attribute of the affected structures.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedRequestBody(pub SignedBody<EventRequestBody>);

impl<'__s> utoipa::ToSchema<'__s> for SignedRequestBody {
    fn schema() -> (
        &'__s str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        let schema_event = EventRequestBody::schema();
        let schema_signature = SignatureBody::schema();
        (
            "SignedRequestBody",
            utoipa::openapi::ObjectBuilder::new()
                .property(schema_event.0, schema_event.1)
                .property(schema_signature.0, schema_signature.1)
                .into(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum EventRequestBody {
    /// Initial event from which the subjects are created
    Create(CreateRequestBody),
    /// Execution events of some of the methods that the smart contract possesses
    Fact(FactRequestBody),
    /// Events that allow the owner of a subject to be modified
    Transfer(TransferRequestBody),
    /// Event that closes the life cycle of a subject
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
    /// Governance identifier
    pub governance_id: String,
    /// Subject schema json identifier
    pub schema_id: String,
    /// Namespace to which the subject belongs
    pub namespace: String,
    /// Name of subject
    pub name: String,
    /// Public key of the subject
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
                ApiError::InvalidParameters(
                    "Invalid DigestIdentifier for governance id".to_string(),
                )
            })?,
            schema_id: self.schema_id,
            namespace: self.namespace,
            name: self.name,
            public_key: KeyIdentifier::from_str(&self.public_key.unwrap()).map_err(|_| {
                ApiError::InvalidParameters("Invalid KeyIdentifier for public key".to_string())
            })?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EOLRequestBody {
    /// Subject identifier
    pub subject_id: String,
}

impl TryInto<EOLRequest> for EOLRequestBody {
    type Error = ApiError;

    fn try_into(self) -> Result<EOLRequest, Self::Error> {
        Ok(EOLRequest {
            subject_id: DigestIdentifier::from_str(&self.subject_id).map_err(|_| {
                ApiError::InvalidParameters("Invalid DigestIdentifier for subject id".to_string())
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
    /// Subject identifier
    pub subject_id: String,
    /// Public key of the new owner
    pub public_key: String,
}

impl TryInto<TransferRequest> for TransferRequestBody {
    type Error = ApiError;

    fn try_into(self) -> Result<TransferRequest, Self::Error> {
        Ok(TransferRequest {
            subject_id: DigestIdentifier::from_str(&self.subject_id).map_err(|_| {
                ApiError::InvalidParameters("Invalid DigestIdentifier for subject id".to_string())
            })?,
            public_key: KeyIdentifier::from_str(&self.public_key).map_err(|_| {
                ApiError::InvalidParameters("Invalid KeyIdentifier for public key".to_string())
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
    /// Subject identifier
    pub subject_id: String,
    /// Changes to be applied to the subject
    pub payload: Value,
}

impl From<FactRequest> for FactRequestBody {
    fn from(value: FactRequest) -> Self {
        FactRequestBody {
            subject_id: value.subject_id.to_str(),
            payload: value.payload.0,
        }
    }
}

impl TryInto<FactRequest> for FactRequestBody {
    type Error = ApiError;
    fn try_into(self) -> Result<FactRequest, Self::Error> {
        Ok(FactRequest {
            subject_id: DigestIdentifier::from_str(&self.subject_id).map_err(|_| {
                ApiError::InvalidParameters("Invalid DigestIdentifier for subject id".to_string())
            })?,
            payload: ValueWrapper(self.payload),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthorizeSubjectBody {
    /// Providers acting on a specific subject
    pub providers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostEventRequestBodyPreSignature {
    /// Type of event request
    pub request: EventRequestBody,
    /// Signature of the issuer
    pub signature: Option<SignatureBody>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SignatureBody {
    /// Signature of the issuer
    signer: String, // KeyIdentifier
    /// Timestamp at which the signature was made
    timestamp: u64,
    /// Signature value
    value: String, // SignatureIdentifier,
    content_hash: String
}

impl From<Signature> for SignatureBody {
    fn from(value: Signature) -> Self {
        Self {
            signer: value.signer.to_str(),
            timestamp: value.timestamp.0,
            value: value.value.to_str(),
            content_hash: value.content_hash.to_str()
        }
    }
}

impl TryInto<Signature> for SignatureBody {
    type Error = ApiError;

    fn try_into(self) -> Result<Signature, Self::Error> {
        Ok(Signature {
            signer: KeyIdentifier::from_str(&self.signer).map_err(|_| {
                ApiError::InvalidParameters(
                    "Invalid KeyIdentifier for signature signer".to_string(),
                )
            })?,
            timestamp: TimeStamp(self.timestamp),
            value: SignatureIdentifier::from_str(&self.value).map_err(|_| {
                ApiError::InvalidParameters("Invalid SignatureIdentifier for signature".to_string())
            })?,
            content_hash: DigestIdentifier::from_str(&self.content_hash).map_err(|_| {
                ApiError::InvalidParameters("Invalid SignatureIdentifier for signature".to_string())
            })?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "state")]
pub enum PatchVoteBody {
    /// Vote to accept a particular request
    RespondedAccepted,
    /// Vote to reject a particular request
    RespondedRejected,
}
