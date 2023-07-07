use serde::Deserialize;
use taple_core::KeyDerivator;
use utoipa::{IntoParams, ToSchema};
#[derive(Debug, Clone, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetAllSubjectsQuery {
    /// Subject from which the query is made (being excluded)
    pub from: Option<String>,
    /// Number of entries
    pub quantity: Option<i64>,
    /// Type of subject (governance, all)
    pub subject_type: Option<String>,
    /// Governance identifier
    pub governanceid: Option<String>
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetWithPagination {
    /// Event from which the query is made (being excluded)
    pub from: Option<i64>,
    /// Number of entries
    pub quantity: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetWithPaginationString {
    /// Subject from which the query is made (being excluded)
    pub from: Option<String>,
    /// Number of entries
    pub quantity: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct AddKeysQuery {
    pub algorithm: Option<KeyAlgorithms>
}

#[derive(Debug, Clone, PartialEq, Deserialize, ToSchema)]
pub enum KeyAlgorithms {
    /// Ed25519 algorithm
    Ed25519,
    /// Secp256k1 algorithm
    Secp256k1,
}

impl Into<KeyDerivator> for KeyAlgorithms {
    fn into(self) -> KeyDerivator {
        match self {
            KeyAlgorithms::Ed25519 => KeyDerivator::Ed25519,
            KeyAlgorithms::Secp256k1 => KeyDerivator::Secp256k1
        }
    }
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetApprovalsQuery {
    /// Status of approvals
    pub status: Option<String>,
    /// Request for approval from which the query is made (being excluded)
    pub from: Option<String>,
    /// Number of entries
    pub quantity: Option<i64>,
}
