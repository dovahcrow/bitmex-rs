use std::collections::BTreeMap;

use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralPositionResponse {
    account: u64,
    symbol: String,
    currency: String,
    underlying: String,
    quote_currency: String,
    commission: f64,
    init_margin_req: f64,
    maint_margin_req: f64,
    risk_limit: f64,
    leverage: f64,
    cross_margin: bool,
    deleverage_percentile: f64,
    rebalanced_pnl: f64,
    prev_realised_pnl: f64,
    prev_unrealised_pnl: f64,
    prev_close_price: f64,
    opening_timestamp: DateTime<Utc>,
    opening_qty: f64,
    opening_cost: f64,
    opening_comm: f64,
    open_order_buy_qty: f64,
    open_order_buy_cost: f64,
    open_order_buy_premium: f64,
    open_order_sell_qty: f64,
    open_order_sell_cost: f64,
    open_order_sell_premium: f64,
    exec_buy_qty: f64,
    exec_buy_cost: f64,
    exec_sell_qty: f64,
    exec_sell_cost: f64,
    exec_qty: f64,
    exec_cost: f64,
    exec_comm: f64,
    current_timestamp: DateTime<Utc>,
    current_qty: f64,
    current_cost: f64,
    current_comm: f64,
    realised_cost: f64,
    unrealised_cost: f64,
    gross_open_cost: f64,
    gross_open_premium: f64,
    gross_exec_cost: f64,
    is_open: bool,
    mark_price: f64,
    mark_value: f64,
    risk_value: f64,
    home_notional: f64,
    foreign_notional: f64,
    pos_state: String,
    pos_cost: f64,
    pos_cost2: f64,
    pos_cross: f64,
    pos_init: f64,
    pos_comm: f64,
    pos_loss: f64,
    pos_margin: f64,
    pos_maint: f64,
    pos_allowance: f64,
    taxable_margin: f64,
    init_margin: f64,
    maint_margin: f64,
    session_margin: f64,
    target_excess_margin: f64,
    var_margin: f64,
    realised_gross_pnl: f64,
    realised_tax: f64,
    realised_pnl: f64,
    unrealised_gross_pnl: f64,
    long_bankrupt: f64,
    short_bankrupt: f64,
    tax_base: f64,
    indicative_tax_rate: f64,
    indicative_tax: f64,
    unrealised_tax: f64,
    unrealised_pnl: f64,
    unrealised_pnl_pcnt: f64,
    unrealised_roe_pcnt: f64,
    simple_qty: f64,
    simple_cost: f64,
    simple_value: f64,
    simple_pnl: f64,
    simple_pnl_pcnt: f64,
    avg_cost_price: f64,
    avg_entry_price: f64,
    break_even_price: f64,
    margin_call_price: f64,
    liquidation_price: f64,
    bankrupt_price: f64,
    timestamp: DateTime<Utc>,
    last_price: f64,
    last_value: f64,
}

#[derive(Clone, Debug, Serialize)]
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
