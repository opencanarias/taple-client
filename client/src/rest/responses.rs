use std::collections::HashSet;

use crate::rest::bodys::SignatureBody;
use serde::{Deserialize, Serialize};
use taple_core::identifier::Derivable;
use taple_core::request::{RequestState, TapleRequest};
use taple_core::KeyIdentifier;
use taple_core::ValueWrapper;
use taple_core::{
    ApprovalEntity, ApprovalRequest, ApprovalResponse, ApprovalState, EvaluationResponse, Event,
    SubjectData,
};
use taple_core::{DigestIdentifier, ValidationProof};
use utoipa::ToSchema;

use super::bodys::EventRequestBody;
use super::bodys::SignedBody;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EventContentResponse {
    pub subject_id: String,
    pub event_request: SignedBody<EventRequestBody>,
    pub sn: u64,
    pub patch: ValueWrapper,
    pub state_hash: String,
    pub eval_success: bool,
    pub appr_required: bool,
    pub approved: bool,
    pub hash_prev_event: String,
    pub evaluators: Vec<SignatureBody>,
    pub approvers: Vec<SignatureBody>,
}

impl From<Event> for EventContentResponse {
    fn from(value: Event) -> Self {
        Self {
            subject_id: value.subject_id.to_str(),
            event_request: SignedBody::<EventRequestBody>::from(value.event_request),
            sn: value.sn,
            patch: value.patch,
            state_hash: value.state_hash.to_str(),
            eval_success: value.eval_success,
            appr_required: value.appr_required,
            approved: value.approved,
            hash_prev_event: value.hash_prev_event.to_str(),
            evaluators: value.evaluators.into_iter().map(|s| s.into()).collect(),
            approvers: value.approvers.into_iter().map(|s| s.into()).collect(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct SubjectDataResponse {
    /// Subject identifier
    pub subject_id: String, // DigestIdentifier
    /// Governance identifier
    pub governance_id: String, // DigestIdentifier
    /// Current sequence number of the subject
    pub sn: u64,
    /// Public key of the subject
    pub public_key: String, // KeyIdentifier
    pub namespace: String,
    /// Identifier of the schema used by the subject and defined in associated governance
    pub schema_id: String,
    /// Subject owner identifier
    pub owner: String, // KeyIdentifier
    /// Subject creator identifier
    pub creator: String, // KeyIdentifier
    /// Current status of the subject
    pub properties: ValueWrapper,
    /// Indicates if the subject is active or not
    pub active: bool,
}

impl From<SubjectData> for SubjectDataResponse {
    fn from(value: SubjectData) -> Self {
        Self {
            subject_id: value.subject_id.to_str(),
            governance_id: value.governance_id.to_str(),
            sn: value.sn,
            public_key: value.public_key.to_str(),
            namespace: value.namespace,
            schema_id: value.schema_id,
            owner: value.owner.to_str(),
            creator: value.creator.to_str(),
            properties: value.properties,
            active: value.active,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ApprovalRequestResponse {
    // Evaluation Request
    pub event_request: SignedBody<EventRequestBody>,
    pub sn: u64,
    pub gov_version: u64,
    // Evaluation Response
    pub patch: ValueWrapper, // cambiar
    pub state_hash: String,
    pub hash_prev_event: String,
}

impl From<ApprovalRequest> for ApprovalRequestResponse {
    fn from(value: ApprovalRequest) -> Self {
        Self {
            event_request: value.event_request.into(),
            sn: value.sn,
            gov_version: value.gov_version,
            patch: value.patch,
            state_hash: value.state_hash.to_str(),
            hash_prev_event: value.hash_prev_event.to_str(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ApprovalResponseBody {
    pub appr_req_hash: String,
    pub approved: bool,
}

impl From<ApprovalResponse> for ApprovalResponseBody {
    fn from(value: ApprovalResponse) -> Self {
        Self {
            appr_req_hash: value.appr_req_hash.to_str(),
            approved: value.approved,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub enum ApprovalStateResponse {
    Pending,
    Responded,
    Obsolete,
}

impl From<ApprovalState> for ApprovalStateResponse {
    fn from(value: ApprovalState) -> Self {
        match value {
            ApprovalState::Pending => Self::Pending,
            ApprovalState::Responded => Self::Responded,
            ApprovalState::Obsolete => Self::Obsolete,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApprovalEntityResponse {
    pub id: String,
    pub request: SignedBody<ApprovalRequestResponse>,
    pub reponse: Option<SignedBody<ApprovalResponseBody>>,
    pub state: ApprovalStateResponse,
}

impl From<ApprovalEntity> for ApprovalEntityResponse {
    fn from(value: ApprovalEntity) -> Self {
        Self {
            id: value.id.to_str(),
            request: value.request.into(),
            reponse: value.reponse.map(|x| x.into()),
            state: value.state.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TapleRequestResponse {
    #[serde(flatten)]
    pub request: EventRequestBody,
    pub signature: SignatureBody,
}

impl From<TapleRequest> for TapleRequestResponse {
    fn from(value: TapleRequest) -> Self {
        let request = value.event_request;
        Self {
            request: request.content.try_into().unwrap(),
            signature: request.signature.try_into().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TapleRequestStateResponse {
    id: String,
    subject_id: Option<String>,
    sn: Option<u64>,
    state: RequestStateResponse,
}

impl From<TapleRequest> for TapleRequestStateResponse {
    fn from(value: TapleRequest) -> Self {
        Self {
            id: value.id.to_str(),
            subject_id: value.subject_id.map(|id| id.to_str()),
            sn: value.sn,
            state: value.state.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum RequestStateResponse {
    #[serde(rename = "finished")]
    Finished,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "processing")]
    Processing,
}

impl From<RequestState> for RequestStateResponse {
    fn from(value: RequestState) -> Self {
        match value {
            RequestState::Finished => Self::Finished,
            RequestState::Error => Self::Error,
            RequestState::Processing => Self::Processing,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ValidationProofResponse {
    pub subject_id: String,
    pub schema_id: String,
    pub namespace: String,
    pub name: String,
    pub subject_public_key: String,
    pub governance_id: String,
    pub genesis_governance_version: u64,
    pub sn: u64,
    pub prev_event_hash: String,
    pub event_hash: String,
    pub governance_version: u64,
}

impl From<ValidationProof> for ValidationProofResponse {
    fn from(value: ValidationProof) -> Self {
        Self {
            subject_id: value.subject_id.to_str(),
            schema_id: value.schema_id,
            namespace: value.namespace,
            name: value.name,
            subject_public_key: value.subject_public_key.to_str(),
            governance_id: value.governance_id.to_str(),
            genesis_governance_version: value.governance_version,
            sn: value.sn,
            prev_event_hash: value.prev_event_hash.to_str(),
            event_hash: value.event_hash.to_str(),
            governance_version: value.governance_version,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GetProofResponse {
    pub proof: ValidationProofResponse,
    pub signatures: Vec<SignatureBody>
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PreauthorizedSubjectsResponse {
    subject_id: String,
    providers: Vec<String>,
}

impl From<(DigestIdentifier, HashSet<KeyIdentifier>)> for PreauthorizedSubjectsResponse {
    fn from(value: (DigestIdentifier, HashSet<KeyIdentifier>)) -> Self {
        Self {
            subject_id: value.0.to_str(),
            providers: value.1.into_iter().map(|i| i.to_str()).collect(),
        }
    }
}
