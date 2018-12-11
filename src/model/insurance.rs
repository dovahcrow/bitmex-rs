use super::GeneralRequest;
use chrono::{DateTime, Utc};
use serde_derive::Deserialize;

pub type GetInsuranceRequest = GeneralRequest;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInsuranceResponse {
    pub currency: String,
    pub timestamp: DateTime<Utc>,
    pub wallet_balance: f64,
}
