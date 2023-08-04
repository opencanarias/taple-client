use std::collections::HashSet;

use crate::rest::bodys::SignatureBody;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use taple_core::identifier::Derivable;
use taple_core::request::{RequestState, TapleRequest};
use taple_core::KeyIdentifier;
use taple_core::{
    ApprovalEntity, ApprovalRequest, ApprovalResponse, ApprovalState, Event, SubjectData,
};
use taple_core::{DigestIdentifier, ValidationProof};
use utoipa::ToSchema;

use super::bodys::SignedBody;
use super::bodys::{EventRequestBody, SignedRequestBody};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedEvent(pub SignedBody<EventContentResponse>);

impl<'__s> utoipa::ToSchema<'__s> for SignedEvent {
    fn schema() -> (
        &'__s str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        let schema_event = EventContentResponse::schema();
        let schema_signature = SignatureBody::schema();
        (
            "SignedEvent",
            utoipa::openapi::ObjectBuilder::new()
                .property(schema_event.0, schema_event.1)
                .property(schema_signature.0, schema_signature.1)
                .into(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EventContentResponse {
    /// Subject identifier
    pub subject_id: String,
    /// Signature of the event request
    pub event_request: SignedRequestBody,
    /// The version of the governance contract.
    pub gov_version: u64,
    /// Current sequence number of the subject
    pub sn: u64,
    /// Changes to be applied to the subject
    pub patch: Value,
    /// Hash of the state
    pub state_hash: String,
    /// Value specifying if the evaluation process has gone well
    pub eval_success: bool,
    /// Value specifying if approval is required
    pub appr_required: bool,
    /// Value specifying if it has been approved
    pub approved: bool,
    /// Previous event hash
    pub hash_prev_event: String,
    /// Signatures of the evaluators
    pub evaluators: Vec<SignatureBody>,
    /// Signatures of the approvers
    pub approvers: Vec<SignatureBody>,
}

impl From<Event> for EventContentResponse {
    fn from(value: Event) -> Self {
        Self {
            subject_id: value.subject_id.to_str(),
            event_request: SignedRequestBody(SignedBody::<EventRequestBody>::from(
                value.event_request,
            )),
            sn: value.sn,
            patch: value.patch.0,
            state_hash: value.state_hash.to_str(),
            eval_success: value.eval_success,
            appr_required: value.appr_required,
            approved: value.approved,
            hash_prev_event: value.hash_prev_event.to_str(),
            evaluators: value.evaluators.into_iter().map(|s| s.into()).collect(),
            approvers: value.approvers.into_iter().map(|s| s.into()).collect(),
            gov_version: value.gov_version,
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
    /// Namespace of the subject
    pub namespace: String,
    /// The name of the subject.
    pub name: String,
    /// Identifier of the schema used by the subject and defined in associated governance
    pub schema_id: String,
    /// Subject owner identifier
    pub owner: String, // KeyIdentifier
    /// Subject creator identifier
    pub creator: String, // KeyIdentifier
    /// Current status of the subject
    pub properties: Value,
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
            properties: value.properties.0,
            active: value.active,
            name: value.name,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ApprovalRequestResponse {
    // Evaluation Request
    /// Signature of the event request
    pub event_request: SignedRequestBody,
    /// Current sequence number of the subject
    pub sn: u64,
    /// Governance version
    pub gov_version: u64,
    // Evaluation Response
    /// Changes to be applied to the subject
    pub patch: Value,
    /// Hash of the state
    pub state_hash: String,
    /// Previous event hash
    pub hash_prev_event: String,
}

impl From<ApprovalRequest> for ApprovalRequestResponse {
    fn from(value: ApprovalRequest) -> Self {
        Self {
            event_request: SignedRequestBody(value.event_request.into()),
            sn: value.sn,
            gov_version: value.gov_version,
            patch: value.patch.0,
            state_hash: value.state_hash.to_str(),
            hash_prev_event: value.hash_prev_event.to_str(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ApprovalResponseBody {
    /// Hash of the request for approval
    pub appr_req_hash: String,
    /// Value specifying if it has been approved
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
    /// Request for approval which is in pending status
    Pending,
    /// Request for approval which is in responded status and accepted
    RespondedAccepted,
    /// Request for approval which is in responded status and rejected
    RespondedRejected,
    /// Request for approval that is obsolete due to a subject update
    Obsolete,
}

impl From<ApprovalState> for ApprovalStateResponse {
    fn from(value: ApprovalState) -> Self {
        match value {
            ApprovalState::Pending => Self::Pending,
            ApprovalState::RespondedAccepted => Self::RespondedAccepted,
            ApprovalState::RespondedRejected => Self::RespondedRejected,
            ApprovalState::Obsolete => Self::Obsolete,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedApprovalRequestResponse(pub SignedBody<ApprovalRequestResponse>);

impl<'__s> utoipa::ToSchema<'__s> for SignedApprovalRequestResponse {
    fn schema() -> (
        &'__s str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        let schema_approval = ApprovalRequestResponse::schema();
        let schema_signature = SignatureBody::schema();
        (
            "SignedApprovalRequestResponse",
            utoipa::openapi::ObjectBuilder::new()
                .property(schema_approval.0, schema_approval.1)
                .property(schema_signature.0, schema_signature.1)
                .into(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedApprovalResponseBody(pub SignedBody<ApprovalResponseBody>);

impl<'__s> utoipa::ToSchema<'__s> for SignedApprovalResponseBody {
    fn schema() -> (
        &'__s str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        let schema_approval = ApprovalResponseBody::schema();
        let schema_signature = SignatureBody::schema();
        (
            "SignedApprovalResponseBody",
            utoipa::openapi::ObjectBuilder::new()
                .property(schema_approval.0, schema_approval.1)
                .property(schema_signature.0, schema_signature.1)
                .into(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApprovalEntityResponse {
    /// Approval request identifier
    pub id: String,
    /// Signature of the request for approval
    pub request: SignedApprovalRequestResponse,
    /// Signature of the petition by approvers
    pub reponse: Option<SignedApprovalResponseBody>,
    /// Current status of the request
    pub state: ApprovalStateResponse,
}

impl From<ApprovalEntity> for ApprovalEntityResponse {
    fn from(value: ApprovalEntity) -> Self {
        Self {
            id: value.id.to_str(),
            request: SignedApprovalRequestResponse(value.request.into()),
            reponse: value.response.map(|x| SignedApprovalResponseBody(x.into())),
            state: value.state.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TapleRequestResponse {
    #[serde(flatten)]
    /// Type of event issued to taple
    pub request: EventRequestBody,
    /// Signature of the person who issued the event
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
    /// Request identifier
    id: String,
    /// Subject identifier
    subject_id: Option<String>,
    /// Current sequence number of the subject
    sn: Option<u64>,
    /// Current status of the request
    state: RequestStateResponse,
    /// Value that says if the request has been successful
    success: Option<bool>,
}

impl From<TapleRequest> for TapleRequestStateResponse {
    fn from(value: TapleRequest) -> Self {
        Self {
            id: value.id.to_str(),
            subject_id: value.subject_id.map(|id| id.to_str()),
            sn: value.sn,
            state: value.state.into(),
            success: value.success,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum RequestStateResponse {
    /// Request has been successfully completed
    #[serde(rename = "finished")]
    Finished,
    /// Request has encountered a problem
    #[serde(rename = "error")]
    Error,
    /// Reques is being processed
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
    /// Subject identifier
    pub subject_id: String,
    /// Subject schema json identifier
    pub schema_id: String,
    /// Namespace to which the subject belongs
    pub namespace: String,
    /// Name of subject
    pub name: String,
    /// Public key of the subject
    pub subject_public_key: String,
    /// Governance identifier
    pub governance_id: String,
    /// Governance version of the genesis event
    pub genesis_governance_version: u64,
    /// Current sequence number of the subject
    pub sn: u64,
    /// Previous event hash
    pub prev_event_hash: String,
    /// Hash of the event
    pub event_hash: String,
    /// Governance version
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
    /// Current validation proof
    pub proof: ValidationProofResponse,
    /// Validators' signatures
    pub signatures: Vec<SignatureBody>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PreauthorizedSubjectsResponse {
    /// Subject identifier
    subject_id: String, // DigestIdentifier
    /// Providers acting on a specific subject
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    /// Error code
    pub code: u16,
    /// Error message
    pub error: String,
}
