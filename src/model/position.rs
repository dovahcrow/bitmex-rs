use std::collections::BTreeMap;

use serde_derive::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralPositionResponse {
    pub account: u64,
    pub symbol: String,
    pub currency: String,
    pub underlying: String,
    pub quote_currency: String,
    pub commission: f64,
    pub init_margin_req: f64,
    pub maint_margin_req: f64,
    pub risk_limit: f64,
    pub leverage: f64,
    pub cross_margin: bool,
    pub deleverage_percentile: Option<f64>,
    pub rebalanced_pnl: f64,
    pub prev_realised_pnl: f64,
    pub prev_unrealised_pnl: f64,
    pub prev_close_price: f64,
    pub opening_timestamp: DateTime<Utc>,
    pub opening_qty: f64,
    pub opening_cost: f64,
    pub opening_comm: f64,
    pub open_order_buy_qty: f64,
    pub open_order_buy_cost: f64,
    pub open_order_buy_premium: f64,
    pub open_order_sell_qty: f64,
    pub open_order_sell_cost: f64,
    pub open_order_sell_premium: f64,
    pub exec_buy_qty: f64,
    pub exec_buy_cost: f64,
    pub exec_sell_qty: f64,
    pub exec_sell_cost: f64,
    pub exec_qty: f64,
    pub exec_cost: f64,
    pub exec_comm: f64,
    pub current_timestamp: DateTime<Utc>,
    pub current_qty: f64,
    pub current_cost: f64,
    pub current_comm: f64,
    pub realised_cost: f64,
    pub unrealised_cost: f64,
    pub gross_open_cost: f64,
    pub gross_open_premium: f64,
    pub gross_exec_cost: f64,
    pub is_open: bool,
    pub mark_price: Option<f64>,
    pub mark_value: f64,
    pub risk_value: f64,
    pub home_notional: f64,
    pub foreign_notional: f64,
    pub pos_state: String,
    pub pos_cost: f64,
    pub pos_cost2: f64,
    pub pos_cross: f64,
    pub pos_init: f64,
    pub pos_comm: f64,
    pub pos_loss: f64,
    pub pos_margin: f64,
    pub pos_maint: f64,
    pub pos_allowance: f64,
    pub taxable_margin: f64,
    pub init_margin: f64,
    pub maint_margin: f64,
    pub session_margin: f64,
    pub target_excess_margin: f64,
    pub var_margin: f64,
    pub realised_gross_pnl: f64,
    pub realised_tax: f64,
    pub realised_pnl: f64,
    pub unrealised_gross_pnl: f64,
    pub long_bankrupt: f64,
    pub short_bankrupt: f64,
    pub tax_base: f64,
    pub indicative_tax_rate: f64,
    pub indicative_tax: f64,
    pub unrealised_tax: f64,
    pub unrealised_pnl: f64,
    pub unrealised_pnl_pcnt: f64,
    pub unrealised_roe_pcnt: f64,
    pub simple_qty: Option<f64>,
    pub simple_cost: Option<f64>,
    pub simple_value: Option<f64>,
    pub simple_pnl: Option<f64>,
    pub simple_pnl_pcnt: Option<f64>,
    pub avg_cost_price: Option<f64>,
    pub avg_entry_price: Option<f64>,
    pub break_even_price: Option<f64>,
    pub margin_call_price: Option<f64>,
    pub liquidation_price: Option<f64>,
    pub bankrupt_price: Option<f64>,
    pub timestamp: DateTime<Utc>,
    pub last_price: Option<f64>,
    pub last_value: f64,
}

#[derive(Clone, Default, Debug, Serialize)]
pub struct GetPositionRequest {
    pub filter: Option<BTreeMap<String, String>>,
    pub columns: Option<u64>,
    pub count: Option<u64>,
}

pub type GetPositionResponse = GeneralPositionResponse;

#[derive(Clone, Default, Debug, Serialize)]
pub struct PostPositionIsolateRequest {
    pub symbol: String,
    pub enabled: bool,
}

pub type PostPositionIsolateResponse = GeneralPositionResponse;

#[derive(Clone, Default, Debug, Serialize)]
pub struct PostPositionLeverageRequest {
    pub symbol: String,
    pub leverage: f64,
}

pub type PostPositionLeverageResponse = GeneralPositionResponse;

#[derive(Clone, Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostPositionRiskLimitRequest {
    pub symbol: String,
    pub risk_limit: f64,
}

pub type PostPositionRiskLimitResponse = GeneralPositionResponse;

#[derive(Clone, Default, Debug, Serialize)]
pub struct PostPositionTransferMarginRequest {
    pub symbol: String,
    pub amount: f64,
}

pub type PostPositionTransferMarginResponse = GeneralPositionResponse;

// // "(\w+)": "String"  $1: String
// // "(\w+)": 0  $1: f64
// // "(\w+)": true  $1: bool
// // "(\w+)": "2018-09-02T18:57:54.593Z"  $1: String
