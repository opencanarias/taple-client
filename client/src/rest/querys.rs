use serde::Deserialize;
use taple_core::KeyDerivator;
use utoipa::{IntoParams, ToSchema};
#[derive(Debug, Clone, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetAllSubjectsQuery {
    pub from: Option<String>,
    pub quantity: Option<i64>,
    pub subject_type: Option<String>,
    pub governanceid: Option<String>
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetWithPagination {
    pub from: Option<i64>,
    pub quantity: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct AddKeysQuery {
    pub algorithm: Option<KeyAlgorithms>
}

#[derive(Debug, Clone, PartialEq, Deserialize, ToSchema)]
pub enum KeyAlgorithms {
    Ed25519,
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
    pub status: Option<String>,
    pub from: Option<String>,
    pub quantity: Option<i64>,
}
