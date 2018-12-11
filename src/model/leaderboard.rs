use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub enum RankingType {
    #[serde(rename = "notional")]
    Notional,
    #[serde(rename = "ROE")]
    ROE,
}

impl Default for RankingType {
    fn default() -> Self {
        self::RankingType::Notional
    }
}

#[derive(Clone, Default, Debug, Serialize)]
pub struct GetLeaderboardRequest {
    pub method: RankingType,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLeaderboardResponse {
    pub name: String,
    pub is_real_name: bool,
    pub profit: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetLeaderboardNameResponse {
    pub name: String,
}
