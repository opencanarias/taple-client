use serde::{Deserialize, Serialize};
use taple_core::identifier::Derivable;
use taple_core::signature::{Signature, SignatureContent};
use taple_core::{
    Acceptance, Approval, ApprovalContent, ApprovalPetitionData, Evaluation, Event, EventContent,
    EventProposal, Proposal, SubjectData, SignatureIdentifier,
};
use utoipa::ToSchema;

use crate::rest::bodys::PostEventRequestBody;
use crate::rest::bodys::SignatureRequest;

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
    pub preevaluation_hash: String,
    pub state_hash: String,
    pub governance_version: u64,
    pub acceptance: AcceptanceResponse,
    pub approval_required: bool,
}

impl From<Evaluation> for EvaluationResponse {
    fn from(value: Evaluation) -> Self {
        Self {
            preevaluation_hash: value.preevaluation_hash.to_str(),
            state_hash: value.state_hash.to_str(),
            governance_version: value.governance_version,
            acceptance: value.acceptance.into(),
            approval_required: value.approval_required,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProposalResponse {
    event_request: PostEventRequestBody,
    sn: u64,
    hash_prev_event: String,
    gov_version: u64,
    evaluation: Option<Evaluation>,
    json_patch: String,
    evaluation_signatures: Vec<SignatureRequest>,
}

impl From<Proposal> for ProposalResponse {
    fn from(value: Proposal) -> Self {
        Self {
            event_request: value.event_request.try_into().unwrap(),
            sn: value.sn,
            hash_prev_event: value.hash_prev_event.to_str(),
            gov_version: value.gov_version,
            evaluation: value.evaluation,
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

impl From<EventProposal> for EventProposalResponse {
    fn from(value: EventProposal) -> Self {
        Self {
            proposal: value.proposal.into(),
            subject_signature: value.subject_signature.try_into().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EventResponse {
    content: EventContentResponse,
    signature: SignatureRequest,
}

impl From<Event> for EventResponse {
    fn from(value: Event) -> Self {
        Self {
            content: value.content.into(),
            signature: SignatureRequest::try_from(value.signature).unwrap(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EventContentResponse {
    event_proposal: EventProposalResponse,
    approvals: Vec<ApprovalResponse>,
    execution: bool,
}

impl From<EventContent> for EventContentResponse {
    fn from(value: EventContent) -> Self {
        Self {
            event_proposal: value.event_proposal.into(),
            approvals: value.approvals.into_iter().map(|x| x.into()).collect(),
            execution: value.execution,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApprovalResponse {
    content: ApprovalContentResponse,
    signature: SignatureRequest,
}

impl From<Approval> for ApprovalResponse {
    fn from(value: Approval) -> Self {
        Self {
            content: value.content.into(),
            signature: value.signature.try_into().unwrap(),
        }
    }
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
            acceptance: value.acceptance.try_into().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SubjectDataResponse {
    subject_id: String,
    governance_id: String,
    sn: u64,
    public_key: String,
    namespace: String,
    schema_id: String,
    owner: String,
    creator: String,
    properties: String,
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
    pub signature: SignatureIdentifier
}

impl From<Signature> for SignatureDataResponse {
    fn from(value: Signature) -> Self {
        Self {
            content: value.content,
            signature: value.signature
        }
    }
}