use super::GeneralRequest;
use chrono::{DateTime, Utc};

pub type GetQuoteRequest = GeneralRequest;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetQuoteResponse {
    timestamp: DateTime<Utc>,
    symbol: String,
    bid_size: f64,
    bid_price: f64,
    ask_price: f64,
    ask_size: f64,
}

#[derive(Clone, Debug, Serialize)]
pub enum BinSize {
    #[serde(rename = "1m")]
    M1,
    #[serde(rename = "5m")]
    M5,
    #[serde(rename = "1h")]
    H1,
    #[serde(rename = "1d")]
    D1,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetQuoteBucketedRequest {
    partial: bool,
    bin_size: BinSize,
    #[serde(flatten)]
    common: GeneralRequest,
}

pub type GetQuoteBucketedResponse = GetQuoteResponse;
