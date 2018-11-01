use super::GeneralRequest;
use super::Side;

use chrono::{DateTime, Utc};

pub type GetExecutionRequest = GeneralRequest;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExecutionResponse {
    #[serde(rename = "execID")]
    pub exec_id: String,
    #[serde(rename = "orderID")]
    pub order_id: String,
    #[serde(rename = "clOrdID")]
    pub cl_ord_id: String,
    #[serde(rename = "clOrdLinkID")]
    pub cl_ord_link_id: String,
    pub account: u64,
    pub symbol: String,
    pub side: Side,
    pub last_qty: f64,
    pub last_px: f64,
    pub underlying_last_px: f64,
    pub last_mkt: String,
    pub last_liquidity_ind: String,
    pub simple_order_qty: f64,
    pub order_qty: f64,
    pub price: f64,
    pub display_qty: f64,
    pub stop_px: f64,
    pub peg_offset_value: f64,
    pub peg_price_type: String,
    pub currency: String,
    pub settl_currency: String,
    pub exec_type: String,
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
    pub commission: f64,
    pub trade_publish_indicator: String,
    pub multi_leg_reporting_type: String,
    pub text: String,
    #[serde(rename = "trdMatchID")]
    pub trd_match_id: String,
    pub exec_cost: f64,
    pub exec_comm: f64,
    pub home_notional: f64,
    pub foreign_notional: f64,
    pub transact_time: DateTime<Utc>,
    pub timestamp: DateTime<Utc>,
}

pub type GetExecutionTradeHistoryRequest = GeneralRequest;
pub type GetExecutionTradeHistoryResponse = GetExecutionResponse;
