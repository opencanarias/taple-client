use serde::Deserialize;
use utoipa::IntoParams;
#[derive(Debug, Clone, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetAllSubjectsQuery {
    pub from: Option<String>,
    pub quantity: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetEventsOfSubjectQuery {
    pub from: Option<i64>,
    pub quantity: Option<i64>,
}
