use super::GeneralRequest;
use chrono::{DateTime, Utc};
use std::collections::BTreeMap;

pub type GetQuoteRequest = GeneralRequest;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetQuoteResponse {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub bid_size: f64,
    pub bid_price: f64,
    pub ask_price: f64,
    pub ask_size: f64,
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

impl Default for BinSize {
    fn default() -> Self {
        self::BinSize::D1
    }
}

#[derive(Clone, Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetQuoteBucketedRequest {
    pub partial: bool,
    pub bin_size: BinSize,
    pub symbol: Option<String>,
    pub filter: Option<BTreeMap<String, String>>,
    pub columns: Option<Vec<String>>,
    pub count: u64,
    pub start: Option<u64>,
    pub reverse: Option<bool>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

pub type GetQuoteBucketedResponse = GetQuoteResponse;
