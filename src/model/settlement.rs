use chrono::{DateTime, Utc};

use super::GeneralRequest;

pub type GetSettlementRequest = GeneralRequest;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSettlementResponseItem {
    timestamp: DateTime<Utc>,
    symbol: String,
    settlement_type: String,
    settled_price: f64,
    option_strike_price: f64,
    option_underlying_price: f64,
    bankrupt: Option<i64>,
    tax_base: Option<i64>,
    tax_rate: Option<f64>,
}

pub type GetSettlementResponse = Vec<GetSettlementResponseItem>;
