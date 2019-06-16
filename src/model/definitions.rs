#![allow(unused)]

use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

pub use super::public::{
    BinSize, ContingencyType, ExecInst, OrdType, PegPriceType, Side, TimeInForce,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Public Announcements
pub struct Announcement {
    pub id: i32,
    pub link: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub date: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct Error {
    pub error: Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct XAny {}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Persistent API Keys for Developers
pub struct APIKey {
    pub id: String,
    pub secret: String,
    pub name: String,
    pub nonce: i64,
    pub cidr: Option<String>,
    pub permissions: Option<Vec<XAny>>,
    pub enabled: Option<bool>,
    pub user_id: i32,
    pub created: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Trollbox Data
pub struct Chat {
    pub id: Option<i32>,
    pub date: DateTime<Utc>,
    pub user: String,
    pub message: String,
    pub html: String,
    pub from_bot: Option<bool>,
    #[serde(rename = "channelID")]
    pub channel_id: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct ChatChannel {
    pub id: Option<i32>,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct ConnectedUsers {
    pub users: Option<i32>,
    pub bots: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Raw Order and Balance Data
pub struct Execution {
    #[serde(rename = "execID")]
    pub exec_id: Uuid,
    #[serde(rename = "orderID")]
    pub order_id: Option<Uuid>,
    #[serde(rename = "clOrdID")]
    pub cl_ord_id: Option<String>,
    #[serde(rename = "clOrdLinkID")]
    pub cl_ord_link_id: Option<String>,
    pub account: Option<i64>,
    pub symbol: Option<String>,
    pub side: Option<Side>,
    pub last_qty: Option<i64>,
    pub last_px: Option<f64>,
    pub underlying_last_px: Option<f64>,
    pub last_mkt: Option<String>,
    pub last_liquidity_ind: Option<String>,
    pub simple_order_qty: Option<f64>,
    pub order_qty: Option<i64>,
    pub price: Option<f64>,
    pub display_qty: Option<i64>,
    pub stop_px: Option<f64>,
    pub peg_offset_value: Option<f64>,
    pub peg_price_type: Option<PegPriceType>,
    pub currency: Option<String>,
    pub settl_currency: Option<String>,
    pub exec_type: Option<String>,
    pub ord_type: Option<OrdType>,
    pub time_in_force: Option<TimeInForce>,
    pub exec_inst: Option<ExecInst>,
    pub contingency_type: Option<ContingencyType>,
    pub ex_destination: Option<String>,
    pub ord_status: Option<String>,
    pub triggered: Option<String>,
    pub working_indicator: Option<bool>,
    pub ord_rej_reason: Option<String>,
    pub simple_leaves_qty: Option<f64>,
    pub leaves_qty: Option<i64>,
    pub simple_cum_qty: Option<f64>,
    pub cum_qty: Option<i64>,
    pub avg_px: Option<f64>,
    pub commission: Option<f64>,
    pub trade_publish_indicator: Option<String>,
    pub multi_leg_reporting_type: Option<String>,
    pub text: Option<String>,
    #[serde(rename = "trdMatchID")]
    pub trd_match_id: Option<Uuid>,
    pub exec_cost: Option<i64>,
    pub exec_comm: Option<i64>,
    pub home_notional: Option<f64>,
    pub foreign_notional: Option<f64>,
    pub transact_time: Option<DateTime<Utc>>,
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Swap Funding History
pub struct Funding {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub funding_interval: Option<DateTime<Utc>>,
    pub funding_rate: Option<f64>,
    pub funding_rate_daily: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Tradeable Contracts, Indices, and History
pub struct Instrument {
    pub symbol: String,
    pub root_symbol: Option<String>,
    pub state: Option<String>,
    pub typ: Option<String>,
    pub listing: Option<DateTime<Utc>>,
    pub front: Option<DateTime<Utc>>,
    pub expiry: Option<DateTime<Utc>>,
    pub settle: Option<DateTime<Utc>>,
    pub relist_interval: Option<DateTime<Utc>>,
    pub inverse_leg: Option<String>,
    pub sell_leg: Option<String>,
    pub buy_leg: Option<String>,
    pub option_strike_pcnt: Option<f64>,
    pub option_strike_round: Option<f64>,
    pub option_strike_price: Option<f64>,
    pub option_multiplier: Option<f64>,
    pub position_currency: Option<String>,
    pub underlying: Option<String>,
    pub quote_currency: Option<String>,
    pub underlying_symbol: Option<String>,
    pub reference: Option<String>,
    pub reference_symbol: Option<String>,
    pub calc_interval: Option<DateTime<Utc>>,
    pub publish_interval: Option<DateTime<Utc>>,
    pub publish_time: Option<DateTime<Utc>>,
    pub max_order_qty: Option<i64>,
    pub max_price: Option<f64>,
    pub lot_size: Option<i64>,
    pub tick_size: Option<f64>,
    pub multiplier: Option<i64>,
    pub settl_currency: Option<String>,
    pub underlying_to_position_multiplier: Option<i64>,
    pub underlying_to_settle_multiplier: Option<i64>,
    pub quote_to_settle_multiplier: Option<i64>,
    pub is_quanto: Option<bool>,
    pub is_inverse: Option<bool>,
    pub init_margin: Option<f64>,
    pub maint_margin: Option<f64>,
    pub risk_limit: Option<i64>,
    pub risk_step: Option<i64>,
    pub limit: Option<f64>,
    pub capped: Option<bool>,
    pub taxed: Option<bool>,
    pub deleverage: Option<bool>,
    pub maker_fee: Option<f64>,
    pub taker_fee: Option<f64>,
    pub settlement_fee: Option<f64>,
    pub insurance_fee: Option<f64>,
    pub funding_base_symbol: Option<String>,
    pub funding_quote_symbol: Option<String>,
    pub funding_premium_symbol: Option<String>,
    pub funding_timestamp: Option<DateTime<Utc>>,
    pub funding_interval: Option<DateTime<Utc>>,
    pub funding_rate: Option<f64>,
    pub indicative_funding_rate: Option<f64>,
    pub rebalance_timestamp: Option<DateTime<Utc>>,
    pub rebalance_interval: Option<DateTime<Utc>>,
    pub opening_timestamp: Option<DateTime<Utc>>,
    pub closing_timestamp: Option<DateTime<Utc>>,
    pub session_interval: Option<DateTime<Utc>>,
    pub prev_close_price: Option<f64>,
    pub limit_down_price: Option<f64>,
    pub limit_up_price: Option<f64>,
    pub bankrupt_limit_down_price: Option<f64>,
    pub bankrupt_limit_up_price: Option<f64>,
    pub prev_total_volume: Option<i64>,
    pub total_volume: Option<i64>,
    pub volume: Option<i64>,
    pub volume24h: Option<i64>,
    pub prev_total_turnover: Option<i64>,
    pub total_turnover: Option<i64>,
    pub turnover: Option<i64>,
    pub turnover24h: Option<i64>,
    pub home_notional24h: Option<f64>,
    pub foreign_notional24h: Option<f64>,
    pub prev_price24h: Option<f64>,
    pub vwap: Option<f64>,
    pub high_price: Option<f64>,
    pub low_price: Option<f64>,
    pub last_price: Option<f64>,
    pub last_price_protected: Option<f64>,
    pub last_tick_direction: Option<String>,
    pub last_change_pcnt: Option<f64>,
    pub bid_price: Option<f64>,
    pub mid_price: Option<f64>,
    pub ask_price: Option<f64>,
    pub impact_bid_price: Option<f64>,
    pub impact_mid_price: Option<f64>,
    pub impact_ask_price: Option<f64>,
    pub has_liquidity: Option<bool>,
    pub open_interest: Option<i64>,
    pub open_value: Option<i64>,
    pub fair_method: Option<String>,
    pub fair_basis_rate: Option<f64>,
    pub fair_basis: Option<f64>,
    pub fair_price: Option<f64>,
    pub mark_method: Option<String>,
    pub mark_price: Option<f64>,
    pub indicative_tax_rate: Option<f64>,
    pub indicative_settle_price: Option<f64>,
    pub option_underlying_price: Option<f64>,
    pub settled_price: Option<f64>,
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct InstrumentInterval {
    pub intervals: Vec<String>,
    pub symbols: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct IndexComposite {
    pub timestamp: DateTime<Utc>,
    pub symbol: Option<String>,
    pub index_symbol: Option<String>,
    pub reference: Option<String>,
    pub last_price: Option<f64>,
    pub weight: Option<f64>,
    pub logged: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Insurance Fund Data
pub struct Insurance {
    pub currency: String,
    pub timestamp: DateTime<Utc>,
    pub wallet_balance: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Information on Top Users
pub struct Leaderboard {
    pub name: String,
    pub is_real_name: Option<bool>,
    pub profit: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Active Liquidations
pub struct Liquidation {
    #[serde(rename = "orderID")]
    pub order_id: Uuid,
    pub symbol: Option<String>,
    pub side: Option<Side>,
    pub price: Option<f64>,
    pub leaves_qty: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Account Notifications
pub struct GlobalNotification {
    pub id: Option<i32>,
    pub date: DateTime<Utc>,
    pub title: String,
    pub body: String,
    pub ttl: i32,
    pub r#type: Option<String>,
    pub closable: Option<bool>,
    pub persist: Option<bool>,
    pub wait_for_visibility: Option<bool>,
    pub sound: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Placement, Cancellation, Amending, and History
pub struct Order {
    #[serde(rename = "orderID")]
    pub order_id: Uuid,
    #[serde(rename = "clOrdID")]
    pub cl_ord_id: Option<String>,
    #[serde(rename = "clOrdLinkID")]
    pub cl_ord_link_id: Option<String>,
    pub account: Option<i64>,
    pub symbol: Option<String>,
    pub side: Option<Side>,
    pub simple_order_qty: Option<f64>,
    pub order_qty: Option<i64>,
    pub price: Option<f64>,
    pub display_qty: Option<i64>,
    pub stop_px: Option<f64>,
    pub peg_offset_value: Option<f64>,
    pub peg_price_type: Option<PegPriceType>,
    pub currency: Option<String>,
    pub settl_currency: Option<String>,
    pub ord_type: Option<OrdType>,
    pub time_in_force: Option<TimeInForce>,
    pub exec_inst: Option<ExecInst>,
    pub contingency_type: Option<ContingencyType>,
    pub ex_destination: Option<String>,
    pub ord_status: Option<String>,
    pub triggered: Option<String>,
    pub working_indicator: Option<bool>,
    pub ord_rej_reason: Option<String>,
    pub simple_leaves_qty: Option<f64>,
    pub leaves_qty: Option<i64>,
    pub simple_cum_qty: Option<f64>,
    pub cum_qty: Option<i64>,
    pub avg_px: Option<f64>,
    pub multi_leg_reporting_type: Option<String>,
    pub text: Option<String>,
    pub transact_time: Option<DateTime<Utc>>,
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct OrderBookL2 {
    pub symbol: String,
    pub id: i64,
    pub side: Side,
    pub size: Option<i64>,
    pub price: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Summary of Open and Closed Positions
pub struct Position {
    pub account: i64,
    pub symbol: String,
    pub currency: String,
    pub underlying: Option<String>,
    pub quote_currency: Option<String>,
    pub commission: Option<f64>,
    pub init_margin_req: Option<f64>,
    pub maint_margin_req: Option<f64>,
    pub risk_limit: Option<i64>,
    pub leverage: Option<f64>,
    pub cross_margin: Option<bool>,
    pub deleverage_percentile: Option<f64>,
    pub rebalanced_pnl: Option<i64>,
    pub prev_realised_pnl: Option<i64>,
    pub prev_unrealised_pnl: Option<i64>,
    pub prev_close_price: Option<f64>,
    pub opening_timestamp: Option<DateTime<Utc>>,
    pub opening_qty: Option<i64>,
    pub opening_cost: Option<i64>,
    pub opening_comm: Option<i64>,
    pub open_order_buy_qty: Option<i64>,
    pub open_order_buy_cost: Option<i64>,
    pub open_order_buy_premium: Option<i64>,
    pub open_order_sell_qty: Option<i64>,
    pub open_order_sell_cost: Option<i64>,
    pub open_order_sell_premium: Option<i64>,
    pub exec_buy_qty: Option<i64>,
    pub exec_buy_cost: Option<i64>,
    pub exec_sell_qty: Option<i64>,
    pub exec_sell_cost: Option<i64>,
    pub exec_qty: Option<i64>,
    pub exec_cost: Option<i64>,
    pub exec_comm: Option<i64>,
    pub current_timestamp: Option<DateTime<Utc>>,
    pub current_qty: Option<i64>,
    pub current_cost: Option<i64>,
    pub current_comm: Option<i64>,
    pub realised_cost: Option<i64>,
    pub unrealised_cost: Option<i64>,
    pub gross_open_cost: Option<i64>,
    pub gross_open_premium: Option<i64>,
    pub gross_exec_cost: Option<i64>,
    pub is_open: Option<bool>,
    pub mark_price: Option<f64>,
    pub mark_value: Option<i64>,
    pub risk_value: Option<i64>,
    pub home_notional: Option<f64>,
    pub foreign_notional: Option<f64>,
    pub pos_state: Option<String>,
    pub pos_cost: Option<i64>,
    pub pos_cost2: Option<i64>,
    pub pos_cross: Option<i64>,
    pub pos_init: Option<i64>,
    pub pos_comm: Option<i64>,
    pub pos_loss: Option<i64>,
    pub pos_margin: Option<i64>,
    pub pos_maint: Option<i64>,
    pub pos_allowance: Option<i64>,
    pub taxable_margin: Option<i64>,
    pub init_margin: Option<i64>,
    pub maint_margin: Option<i64>,
    pub session_margin: Option<i64>,
    pub target_excess_margin: Option<i64>,
    pub var_margin: Option<i64>,
    pub realised_gross_pnl: Option<i64>,
    pub realised_tax: Option<i64>,
    pub realised_pnl: Option<i64>,
    pub unrealised_gross_pnl: Option<i64>,
    pub long_bankrupt: Option<i64>,
    pub short_bankrupt: Option<i64>,
    pub tax_base: Option<i64>,
    pub indicative_tax_rate: Option<f64>,
    pub indicative_tax: Option<i64>,
    pub unrealised_tax: Option<i64>,
    pub unrealised_pnl: Option<i64>,
    pub unrealised_pnl_pcnt: Option<f64>,
    pub unrealised_roe_pcnt: Option<f64>,
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
    pub timestamp: Option<DateTime<Utc>>,
    pub last_price: Option<f64>,
    pub last_value: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Best Bid/Offer Snapshots & Historical Bins
pub struct Quote {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub bid_size: Option<i64>,
    pub bid_price: Option<f64>,
    pub ask_price: Option<f64>,
    pub ask_size: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Historical Settlement Data
pub struct Settlement {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub settlement_type: Option<String>,
    pub settled_price: Option<f64>,
    pub option_strike_price: Option<f64>,
    pub option_underlying_price: Option<f64>,
    pub bankrupt: Option<i64>,
    pub tax_base: Option<i64>,
    pub tax_rate: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Exchange Statistics
pub struct Stats {
    pub root_symbol: String,
    pub currency: Option<String>,
    pub volume24h: Option<i64>,
    pub turnover24h: Option<i64>,
    pub open_interest: Option<i64>,
    pub open_value: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct StatsHistory {
    pub date: DateTime<Utc>,
    pub root_symbol: String,
    pub currency: Option<String>,
    pub volume: Option<i64>,
    pub turnover: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct StatsUSD {
    pub root_symbol: String,
    pub currency: Option<String>,
    pub turnover24h: Option<i64>,
    pub turnover30d: Option<i64>,
    pub turnover365d: Option<i64>,
    pub turnover: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Individual & Bucketed Trades
pub struct Trade {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub side: Option<Side>,
    pub size: Option<i64>,
    pub price: Option<f64>,
    pub tick_direction: Option<String>,
    #[serde(rename = "trdMatchID")]
    pub trd_match_id: Option<Uuid>,
    pub gross_value: Option<i64>,
    pub home_notional: Option<f64>,
    pub foreign_notional: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct TradeBin {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub trades: Option<i64>,
    pub volume: Option<i64>,
    pub vwap: Option<f64>,
    pub last_size: Option<i64>,
    pub turnover: Option<i64>,
    pub home_notional: Option<f64>,
    pub foreign_notional: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct Wallet {
    pub account: i64,
    pub currency: String,
    pub prev_deposited: Option<i64>,
    pub prev_withdrawn: Option<i64>,
    pub prev_transfer_in: Option<i64>,
    pub prev_transfer_out: Option<i64>,
    pub prev_amount: Option<i64>,
    pub prev_timestamp: Option<DateTime<Utc>>,
    pub delta_deposited: Option<i64>,
    pub delta_withdrawn: Option<i64>,
    pub delta_transfer_in: Option<i64>,
    pub delta_transfer_out: Option<i64>,
    pub delta_amount: Option<i64>,
    pub deposited: Option<i64>,
    pub withdrawn: Option<i64>,
    pub transfer_in: Option<i64>,
    pub transfer_out: Option<i64>,
    pub amount: Option<i64>,
    pub pending_credit: Option<i64>,
    pub pending_debit: Option<i64>,
    pub confirmed_debit: Option<i64>,
    pub timestamp: Option<DateTime<Utc>>,
    pub addr: Option<String>,
    pub script: Option<String>,
    pub withdrawal_lock: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct Transaction {
    #[serde(rename = "transactID")]
    pub transact_id: Uuid,
    pub account: Option<i64>,
    pub currency: Option<String>,
    pub transact_type: Option<String>,
    pub amount: Option<i64>,
    pub fee: Option<i64>,
    pub transact_status: Option<String>,
    pub address: Option<String>,
    pub tx: Option<String>,
    pub text: Option<String>,
    pub transact_time: Option<DateTime<Utc>>,
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct AccessToken {
    pub id: String,
    pub ttl: Option<f64>,
    pub created: Option<DateTime<Utc>>,
    pub user_id: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct Affiliate {
    pub account: i64,
    pub currency: String,
    pub prev_payout: Option<i64>,
    pub prev_turnover: Option<i64>,
    pub prev_comm: Option<i64>,
    pub prev_timestamp: Option<DateTime<Utc>>,
    pub exec_turnover: Option<i64>,
    pub exec_comm: Option<i64>,
    pub total_referrals: Option<i64>,
    pub total_turnover: Option<i64>,
    pub total_comm: Option<i64>,
    pub payout_pcnt: Option<f64>,
    pub pending_payout: Option<i64>,
    pub timestamp: Option<DateTime<Utc>>,
    pub referrer_account: Option<f64>,
    pub referral_discount: Option<f64>,
    pub affiliate_payout: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Account Operations
pub struct User {
    pub id: Option<i32>,
    pub owner_id: Option<i32>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub created: Option<DateTime<Utc>>,
    pub last_updated: Option<DateTime<Utc>>,
    pub preferences: Option<UserPreferences>,
    pub restricted_engine_fields: Option<Value>,
    pub tfa_enabled: Option<String>,
    #[serde(rename = "affiliateID")]
    pub affiliate_id: Option<String>,
    pub pgp_pub_key: Option<String>,
    pub country: Option<String>,
    pub geoip_country: Option<String>,
    pub geoip_region: Option<String>,
    pub typ: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct UserCommission {
    pub maker_fee: Option<f64>,
    pub taker_fee: Option<f64>,
    pub settlement_fee: Option<f64>,
    pub max_fee: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct Margin {
    pub account: i64,
    pub currency: String,
    pub risk_limit: Option<i64>,
    pub prev_state: Option<String>,
    pub state: Option<String>,
    pub action: Option<String>,
    pub amount: Option<i64>,
    pub pending_credit: Option<i64>,
    pub pending_debit: Option<i64>,
    pub confirmed_debit: Option<i64>,
    pub prev_realised_pnl: Option<i64>,
    pub prev_unrealised_pnl: Option<i64>,
    pub gross_comm: Option<i64>,
    pub gross_open_cost: Option<i64>,
    pub gross_open_premium: Option<i64>,
    pub gross_exec_cost: Option<i64>,
    pub gross_mark_value: Option<i64>,
    pub risk_value: Option<i64>,
    pub taxable_margin: Option<i64>,
    pub init_margin: Option<i64>,
    pub maint_margin: Option<i64>,
    pub session_margin: Option<i64>,
    pub target_excess_margin: Option<i64>,
    pub var_margin: Option<i64>,
    pub realised_pnl: Option<i64>,
    pub unrealised_pnl: Option<i64>,
    pub indicative_tax: Option<i64>,
    pub unrealised_profit: Option<i64>,
    pub synthetic_margin: Option<i64>,
    pub wallet_balance: Option<i64>,
    pub margin_balance: Option<i64>,
    pub margin_balance_pcnt: Option<f64>,
    pub margin_leverage: Option<f64>,
    pub margin_used_pcnt: Option<f64>,
    pub excess_margin: Option<i64>,
    pub excess_margin_pcnt: Option<f64>,
    pub available_margin: Option<i64>,
    pub withdrawable_margin: Option<i64>,
    pub timestamp: Option<DateTime<Utc>>,
    pub gross_last_value: Option<i64>,
    pub commission: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// User communication SNS token
pub struct CommunicationToken {
    pub id: String,
    pub user_id: i32,
    pub device_token: String,
    pub channel: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// User Events for auditing
pub struct UserEvent {
    pub id: Option<f64>,
    pub r#type: String,
    pub status: String,
    pub user_id: f64,
    pub created_by_id: f64,
    pub ip: Option<String>,
    pub geoip_country: Option<String>,
    pub geoip_region: Option<String>,
    pub geoip_sub_region: Option<String>,
    pub event_meta: Option<Value>,
    pub created: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct UserPreferences {
    pub alert_on_liquidations: Option<bool>,
    pub animations_enabled: Option<bool>,
    pub announcements_last_seen: Option<DateTime<Utc>>,
    #[serde(rename = "chatChannelID")]
    pub chat_channel_id: Option<f64>,
    pub color_theme: Option<String>,
    pub currency: Option<String>,
    pub debug: Option<bool>,
    pub disable_emails: Option<Vec<String>>,
    pub disable_push: Option<Vec<String>>,
    pub hide_confirm_dialogs: Option<Vec<String>>,
    pub hide_connection_modal: Option<bool>,
    pub hide_from_leaderboard: Option<bool>,
    pub hide_name_from_leaderboard: Option<bool>,
    pub hide_notifications: Option<Vec<String>>,
    pub locale: Option<String>,
    pub msgs_seen: Option<Vec<String>>,
    pub order_book_binning: Option<Value>,
    pub order_book_type: Option<String>,
    pub order_clear_immediate: Option<bool>,
    pub order_controls_plus_minus: Option<bool>,
    pub show_locale_numbers: Option<bool>,
    pub sounds: Option<Vec<String>>,
    pub strict_ip_check: Option<bool>,
    pub strict_timeout: Option<bool>,
    pub ticker_group: Option<String>,
    pub ticker_pinned: Option<bool>,
    pub trade_layout: Option<String>,
}
