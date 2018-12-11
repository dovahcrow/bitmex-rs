use super::{GeneralRequest, Side};
use serde_derive::Deserialize;

pub type GetLiquidationRequest = GeneralRequest;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLiquidationResponse {
    #[serde(rename = "orderID")]
    pub order_id: String,
    pub symbol: String,
    pub side: Side,
    pub price: f64,
    pub leaves_qty: f64,
}
