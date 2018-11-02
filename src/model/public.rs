use chrono::{DateTime, Utc};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralRequest {
    pub symbol: Option<String>,
    pub filter: Option<BTreeMap<String, String>>,
    pub columns: Option<Vec<String>>,
    pub count: u64,
    pub start: Option<u64>,
    pub reverse: Option<bool>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Side {
    Buy,
    Sell,
    #[serde(rename = "")]
    Empty, // BitMEX sometimes has empty side due to unknown reason
}
