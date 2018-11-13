use super::definitions::Quote;
use super::BinSize;
use super::GeneralRequest;
use chrono::{DateTime, Utc};
use std::collections::BTreeMap;

pub type GetQuoteRequest = GeneralRequest;

pub type GetQuoteResponse = Vec<Quote>;

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

pub type GetQuoteBucketedResponse = Vec<Quote>;
