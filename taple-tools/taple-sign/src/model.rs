use std::str::FromStr;

use serde::{Deserialize, Serialize};
use taple_core::{
    request::StartRequest as TCreateRequest, request::EOLRequest as TEOLRequest,
    request::FactRequest as TFactRequest,
    request::TransferRequest as TTreansferRequest, DigestIdentifier, EventRequest,
    KeyIdentifier, ValueWrapper,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventRequestTypeBody {
    Create(CreateRequest),
    Fact(FactRequest),
    Transfer(TransferRequest),
    EOL(EOLRequest),
}

impl Into<EventRequest> for EventRequestTypeBody {
    fn into(self) -> EventRequest {
        match self {
            Self::Create(data) => EventRequest::Create(TCreateRequest {
                governance_id: DigestIdentifier::from_str(&data.governance_id)
                    .expect("Should be DigestIdentifier"),
                schema_id: data.schema_id,
                namespace: data.namespace,
                name: data.name,
                public_key: KeyIdentifier::from_str(&data.public_key)
                    .expect("Should be KeyIdentifier"),
            }),
            Self::Fact(data) => EventRequest::Fact(TFactRequest {
                subject_id: DigestIdentifier::from_str(&data.subject_id)
                    .expect("Should be DigestIdentifier"),
                payload: ValueWrapper(data.payload),
            }),
            Self::Transfer(data) => EventRequest::Transfer(TTreansferRequest {
                subject_id: DigestIdentifier::from_str(&data.subject_id)
                    .expect("Should be DigestIdentifier"),
                public_key: KeyIdentifier::from_str(&data.subject_pub_key)
                    .expect("Should be KeyIdentifier"),
            }),
            Self::EOL(data) => EventRequest::EOL(TEOLRequest {
                subject_id: DigestIdentifier::from_str(&data.subject_id)
                    .expect("Should be DigestIdentifier"),
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequest {
    pub governance_id: String,
    pub schema_id: String,
    pub namespace: String,
    pub name: String,
    pub public_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactRequest {
    pub subject_id: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRequest {
    pub subject_id: String,
    pub subject_pub_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EOLRequest {
    pub subject_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureBody {
    pub signer: String, // KeyIdentifier
    pub timestamp: u64,
    pub value: String, // SignatureIdentifier,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignedEventRequest {
    #[serde(rename = "request")]
    pub content: EventRequestTypeBody,
    pub signature: SignatureBody
}