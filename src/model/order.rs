use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::Side;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum PegPriceType {
    LastPeg,
    MidPricePeg,
    MarketPeg,
    PrimaryPeg,
    TrailingStopPeg,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum OrdType {
    Market,
    Limit,
    Stop,
    StopLimit,
    MarketIfTouched,
    LimitIfTouched,
    MarketWithLeftOverAsLimit,
    Pegged,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum TimeInForce {
    Day,
    GoodTillCancel,
    ImmediateOrCancel,
    FillOrKill,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ExecInst {
    ParticipateDoNotInitiate,
    AllOrNone,
    MarkPrice,
    IndexPrice,
    LastPrice,
    Close,
    ReduceOnly,
    Fixed,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ContingencyType {
    OneCancelsTheOther,
    OneTriggersTheOther,
    OneUpdatesTheOtherAbsolute,
    OneUpdatesTheOtherProportional,
}

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

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostOrderResponse {
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

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PutOrderRequest {
    #[serde(rename = "orderID")]
    pub order_id: Uuid,
    #[serde(rename = "clOrdID")]
    pub cl_ord_id: String,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutOrderResponse {
    #[serde(rename = "orderID")]
    pub order_id: Uuid,
    #[serde(rename = "clOrdID")]
    pub cl_ord_id: String,
    #[serde(rename = "clOrdLinkID")]
    pub cl_ord_link_id: String,
    pub account: f64,
    pub symbol: String,
    pub side: Side,
    pub simple_order_qty: Option<f64>,
    pub order_qty: f64,
    pub price: f64,
    pub display_qty: f64,
    pub stop_px: f64,
    pub peg_offset_value: f64,
    pub peg_price_type: String,
    pub currency: String,
    pub settl_currency: String,
    pub ord_type: String,
    pub time_in_force: String,
    pub exec_inst: String,
    pub contingency_type: String,
    pub ex_destination: String,
    pub ord_status: String,
    pub triggered: String,
    pub working_indicator: bool,
    pub ord_rej_reason: String,
    pub simple_leaves_qty: f64,
    pub leaves_qty: f64,
    pub simple_cum_qty: f64,
    pub cum_qty: f64,
    pub avg_px: f64,
    pub multi_leg_reporting_type: String,
    pub text: String,
    pub transact_time: DateTime<Utc>,
    pub timestamp: DateTime<Utc>,
}

// #[derive(Clone, Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct DeleteOrderRequest {
//     #[serde(rename = "orderID")]
//     order_id: Option<String>,
//     #[serde(rename = "clOrdID")]
//     pub cl_ord_id: Option<String>,
//     text: Option<String>,
// }

// // "(\w+)": "String"  $1: String
// // "(\w+)": 0  $1: f64
// // "(\w+)": true  $1: bool
// // "(\w+)": "2018-09-02T18:57:54.593Z"  $1: String

// #[derive(Clone, Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct DeleteOrderResponse {
//     #[serde(rename = "orderID")]
//     order_id: String,
//     #[serde(rename = "clOrdID")]
//     pub cl_ord_id: Option<String>,
//     clOrdLinkID: String,
//     account: f64,
//     symbol: String,
//     side: String,
//     simpleOrderQty: f64,
//     orderQty: f64,
//     price: f64,
//     displayQty: f64,
//     stopPx: f64,
//     pegOffsetValue: f64,
//     pegPriceType: String,
//     currency: String,
//     settlCurrency: String,
//     ordType: String,
//     timeInForce: String,
//     execInst: String,
//     contingencyType: String,
//     exDestination: String,
//     ordStatus: String,
//     triggered: String,
//     workingIndicator: bool,
//     ordRejReason: String,
//     simpleLeavesQty: f64,
//     leavesQty: f64,
//     simpleCumQty: f64,
//     cumQty: f64,
//     avgPx: f64,
//     multiLegReportingType: String,
//     text: String,
//     transactTime: String,
//     timestamp: String,
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct DeleteOrderAllRequest {
//     symbol: Option<String>,
//     filter: Option<String>,
//     text: Option<String>,
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct DeleteOrderAllResponse {
//     orderID: String,
//     clOrdID: String,
//     clOrdLinkID: String,
//     account: f64,
//     symbol: String,
//     side: String,
//     simpleOrderQty: f64,
//     orderQty: f64,
//     price: f64,
//     displayQty: f64,
//     stopPx: f64,
//     pegOffsetValue: f64,
//     pegPriceType: String,
//     currency: String,
//     settlCurrency: String,
//     ordType: String,
//     timeInForce: String,
//     execInst: String,
//     contingencyType: String,
//     exDestination: String,
//     ordStatus: String,
//     triggered: String,
//     workingIndicator: bool,
//     ordRejReason: String,
//     simpleLeavesQty: f64,
//     leavesQty: f64,
//     simpleCumQty: f64,
//     cumQty: f64,
//     avgPx: f64,
//     multiLegReportingType: String,
//     text: String,
//     transactTime: String,
//     timestamp: String,
// }
