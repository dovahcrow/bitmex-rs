use super::instrument::GetInstrumentRequest;
use chrono::{DateTime, Utc};

pub type GetFundingRequest = GetInstrumentRequest;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingResponse {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub funding_interval: DateTime<Utc>,
    pub funding_rate: f64,
    pub funding_rate_daily: f64,
}
