use crate::rest::bodys::PostEventRequestBody;
use crate::rest::bodys::SignatureRequest;
use serde::{Deserialize, Serialize};
use taple_core::ApiError;
use taple_core::identifier::Derivable;
use taple_core::request::{RequestState, TapleRequest};
use taple_core::signature::{Signature, SignatureContent};
use taple_core::{
    Acceptance, ApprovalContent, ApprovalPetitionData, Evaluation, Event, EventContent,
    EventProposal, Proposal, SignatureIdentifier, SubjectData,
};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum AcceptanceResponse {
    Ok,
    Ko,
}

impl From<Acceptance> for AcceptanceResponse {
    fn from(value: Acceptance) -> Self {
        match value {
            Acceptance::Ko => Self::Ko,
            Acceptance::Ok => Self::Ok,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EvaluationResponse {
    pub preevaluation_hash: String, // DigestIdentifier
    pub state_hash: String,         // DigestIdentifier
    pub governance_version: u64,
    pub acceptance: AcceptanceResponse, // Acceptance
    pub approval_required: bool,
}

impl From<Evaluation> for EvaluationResponse {
    fn from(value: Evaluation) -> Self {
        Self {
            preevaluation_hash: value.preevaluation_hash.to_str(),
            state_hash: value.state_hash.to_str(),
            governance_version: value.governance_version,
            acceptance: AcceptanceResponse::from(value.acceptance),
            approval_required: value.approval_required,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProposalResponse {
    event_request: PostEventRequestBody, // EventRequest
    sn: u64,
    hash_prev_event: String, // DigestIdentifier
    gov_version: u64,
    evaluation: Option<EvaluationResponse>, // Option<Evaluation>
    json_patch: String,
    evaluation_signatures: Vec<SignatureRequest>, // HashSet<Signature>
}

impl From<Proposal> for ProposalResponse {
    fn from(value: Proposal) -> Self {
        Self {
            event_request: value.event_request.try_into().unwrap(),
            sn: value.sn,
            hash_prev_event: value.hash_prev_event.to_str(),
            gov_version: value.gov_version,
            evaluation: Some(EvaluationResponse::from(value.evaluation.unwrap())),
            json_patch: value.json_patch,
            evaluation_signatures: value
                .evaluation_signatures
                .into_iter()
                .map(|x| <Signature as TryInto<SignatureRequest>>::try_into(x).unwrap())
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EventProposalResponse {
    pub proposal: ProposalResponse,
    pub subject_signature: SignatureRequest,
}

impl TryFrom<EventProposal> for EventProposalResponse {
    type Error = ApiError;

    fn try_from(value: EventProposal) -> Result<Self, Self::Error> {
        Ok(Self {
            proposal: ProposalResponse::from(value.proposal),
            subject_signature: value.subject_signature.try_into().unwrap(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EventResponse {
    pub content: EventContentResponse, // Event
    pub signature: SignatureRequest,   // Signature
}

impl TryFrom<Event> for EventResponse {
    type Error = ApiError;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        Ok(Self {
            content: EventContentResponse::try_from(value.content)?,
            signature: SignatureRequest::try_from(value.signature)?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EventContentResponse {
    pub event_proposal: EventProposalResponse, // EventProposal
    pub approvals: Vec<ApprovalResponse>,      // HashSet<Approval>
    pub execution: bool,
}

impl TryFrom<EventContent> for EventContentResponse {
    type Error = ApiError;

    fn try_from(value: EventContent) -> Result<Self, Self::Error> {
        Ok(Self {
            event_proposal: EventProposalResponse::try_from(value.event_proposal)?,
            approvals: value
                .approvals
                .iter()
                .map(|approval| ApprovalResponse {
                    content: ApprovalContentResponse::from(approval.content.clone()),
                    signature: SignatureRequest::try_from(approval.signature.clone()).unwrap(),
                })
                .collect(),
            execution: value.execution,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApprovalResponse {
    pub content: ApprovalContentResponse,
    pub signature: SignatureRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApprovalContentResponse {
    pub event_proposal_hash: String,
    pub acceptance: AcceptanceResponse,
}

impl From<ApprovalContent> for ApprovalContentResponse {
    fn from(value: ApprovalContent) -> Self {
        Self {
            event_proposal_hash: value.event_proposal_hash.to_str(),
            acceptance: AcceptanceResponse::from(value.acceptance),
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
    pub properties: String,
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApprovalPetitionDataResponse {
    subject_id: String,
    sn: u64,
    governance_id: String,
    governance_version: u64,
    hash_event_proporsal: String,
    sender: String,
    json_patch: String,
}

impl From<ApprovalPetitionData> for ApprovalPetitionDataResponse {
    fn from(value: ApprovalPetitionData) -> Self {
        Self {
            subject_id: value.subject_id.to_str(),
            sn: value.sn,
            governance_id: value.governance_id.to_str(),
            governance_version: value.governance_version,
            hash_event_proporsal: value.hash_event_proporsal.to_str(),
            sender: value.sender.to_str(),
            json_patch: value.json_patch,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SignatureDataResponse {
    pub content: SignatureContent,
    pub signature: SignatureIdentifier,
}

impl From<Signature> for SignatureDataResponse {
    fn from(value: Signature) -> Self {
        Self {
            content: value.content,
            signature: value.signature,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TapleRequestResponse {
    id: String,
    subject_id: Option<String>,
    sn: Option<u64>,
    event_request: PostEventRequestBody,
    state: RequestStateResponse,
}

impl From<TapleRequest> for TapleRequestResponse {
    fn from(value: TapleRequest) -> Self {
        Self {
            id: value.id.to_str(),
            subject_id: value.subject_id.map(|id| id.to_str()),
            sn: value.sn,
            event_request: value.event_request.try_into().unwrap(),
            state: value.state.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum RequestStateResponse {
    Finished,
    Error,
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
