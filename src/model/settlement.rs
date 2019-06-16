use chrono::{DateTime, Utc};
use serde_derive::Deserialize;

use super::GeneralRequest;

pub type GetSettlementRequest = GeneralRequest;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSettlementResponseItem {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub settlement_type: String,
    pub settled_price: f64,
    pub option_strike_price: f64,
    pub option_underlying_price: f64,
    pub bankrupt: Option<i64>,
    pub tax_base: Option<i64>,
    pub tax_rate: Option<f64>,
}

pub type GetSettlementResponse = Vec<GetSettlementResponseItem>;
