use super::BinSize;
use super::{
    definitions::{Trade, TradeBin},
    GeneralRequest,
};
use chrono::{DateTime, Utc};
use std::collections::BTreeMap;

pub type GetTradeRequest = GeneralRequest;
pub type GetTradeResponse = Vec<Trade>;

#[derive(Clone, Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTradeBucketedRequest {
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

pub type GetTradeBucketedResponse = Vec<TradeBin>;
