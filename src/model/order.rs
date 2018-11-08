use super::GeneralRequest;
pub use super::{BinSize, ContingencyType, ExecInst, OrdType, PegPriceType, Side, TimeInForce};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralOrderResponse {
    #[serde(rename = "orderID")]
    pub order_id: Uuid,
    #[serde(rename = "clOrdID")]
    pub cl_ord_id: String,
    #[serde(rename = "clOrdLinkID")]
    pub cl_ord_link_id: String,
    pub account: u64,
    pub symbol: String,
    pub side: Side,
    pub simple_order_qty: Option<f64>,
    pub order_qty: f64,
    pub price: Option<f64>,
    pub display_qty: Option<f64>,
    pub stop_px: Option<f64>,
    pub peg_offset_value: Option<f64>,
    pub peg_price_type: String,
    pub currency: String,
    pub settl_currency: String,
    pub ord_type: OrdType,
    pub time_in_force: TimeInForce,
    pub exec_inst: String,
    pub contingency_type: String,
    pub ex_destination: String,
    pub ord_status: String,
    pub triggered: String,
    pub working_indicator: bool,
    pub ord_rej_reason: String,
    pub simple_leaves_qty: Option<f64>,
    pub leaves_qty: f64,
    pub simple_cum_qty: Option<f64>,
    pub cum_qty: f64,
    pub avg_px: Option<f64>,
    pub multi_leg_reporting_type: String,
    pub text: String,
    pub transact_time: DateTime<Utc>,
    pub timestamp: DateTime<Utc>,
}

pub type GetOrderRequest = GeneralRequest;
pub type GetOrderResponse = GeneralOrderResponse;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PutOrderRequest {
    pub symbol: String,
    #[serde(rename = "orderID")]
    pub order_id: Option<Uuid>,
    #[serde(rename = "clOrdID")]
    pub cl_ord_id: Option<String>,
    #[serde(rename = "origClOrdID")]
    pub orig_cl_ord_id: Option<String>,
    pub simple_order_qty: Option<f64>,
    pub order_qty: Option<f64>,
    pub simple_leaves_qty: Option<f64>,
    pub leaves_qty: Option<f64>,
    pub price: Option<f64>,
    pub stop_px: Option<f64>,
    pub peg_offset_value: Option<f64>,
    pub text: Option<String>,
}

pub type PutOrderResponse = GeneralOrderResponse;

#[derive(Clone, Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostOrderRequest {
    pub symbol: String,
    pub side: Option<Side>,
    pub simple_order_qty: Option<f64>,
    pub order_qty: Option<f64>,
    pub price: Option<f64>,
    pub display_qty: Option<f64>,
    pub stop_px: Option<f64>,
    #[serde(rename = "clOrdID")]
    pub cl_ord_id: Option<String>,
    #[serde(rename = "clOrdLinkID")]
    pub cl_ord_link_id: Option<String>,
    pub peg_offset_value: Option<f64>,
    pub peg_price_type: Option<PegPriceType>,
    pub ord_type: Option<OrdType>,
    pub time_in_force: Option<TimeInForce>,
    pub exec_inst: Option<Vec<ExecInst>>,
    pub contingency_type: Option<ContingencyType>,
    pub text: Option<String>,
}

pub type PostOrderResponse = GeneralOrderResponse;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteOrderRequest {
    #[serde(rename = "orderID")]
    pub order_id: Option<Uuid>,
    #[serde(rename = "clOrdID")]
    pub cl_ord_id: Option<String>,
    pub text: Option<String>,
}

pub type DeleteOrderResponse = GeneralOrderResponse;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct DeleteOrderAllRequest {
    pub symbol: Option<String>,
    pub filter: Option<String>,
    pub test: Option<String>,
}

pub type DeleteOrderAllResponse = GeneralOrderResponse;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostOrderCancelAllAfterRequest {
    pub timeout: u64,
}

pub type PostOrderCancelAllAfterResponse = ();
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostOrderClosePositionRequest {
    pub symbol: String,
    pub price: Option<f64>,
}

pub type PostOrderClosePositionResponse = GeneralOrderResponse;

// // "(\w+)": "String"  $1: String
// // "(\w+)": 0  $1: f64
// // "(\w+)": true  $1: bool
// // "(\w+)": "2018-09-02T18:57:54.593Z"  $1: String
