use super::Side;

#[derive(Clone, Debug, Serialize)]
pub struct GetOrderBookL2Request {
    pub symbol: String,
    pub depth: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderBookL2Response {
    pub symbol: String,
    pub id: u64,
    pub side: Side,
    pub size: f64,
    pub price: f64,
}
