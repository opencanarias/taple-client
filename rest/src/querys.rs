use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetEventsQuery {
    // SN of initial event
    pub from: Option<i64>,
    // Quantity of events requested
    pub quantity: Option<i64>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetAllSubjectsQuery {
    // Number of initial subject
    pub from: Option<usize>,
    // Quantity of subjects requested
    pub quantity: Option<usize>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetSignaturesQuery {
    // Number of initial signature
    pub from: Option<usize>,
    // Quantity of signatures requested
    pub quantity: Option<usize>,
}
