#![allow(unused)]
pub use super::public::{BinSize, ContingencyType, ExecInst, OrdType, PegPriceType, Side, TimeInForce};
use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Public Announcements
pub struct Announcement {
    id: i32,
    link: Option<String>,
    title: Option<String>,
    content: Option<String>,
    date: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct Error {
    error: Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct XAny {}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Persistent API Keys for Developers
pub struct APIKey {
    id: String,
    secret: String,
    name: String,
    nonce: i64,
    cidr: Option<String>,
    permissions: Option<Vec<XAny>>,
    enabled: Option<bool>,
    user_id: i32,
    created: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Trollbox Data
pub struct Chat {
    id: Option<i32>,
    date: DateTime<Utc>,
    user: String,
    message: String,
    html: String,
    from_bot: Option<bool>,
    #[serde(rename = "channelID")]
    channel_id: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct ChatChannel {
    id: Option<i32>,
    name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct ConnectedUsers {
    users: Option<i32>,
    bots: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Raw Order and Balance Data
pub struct Execution {
    #[serde(rename = "execID")]
    exec_id: Uuid,
    #[serde(rename = "orderID")]
    order_id: Option<Uuid>,
    #[serde(rename = "clOrdID")]
    cl_ord_id: Option<String>,
    #[serde(rename = "clOrdLinkID")]
    cl_ord_link_id: Option<String>,
    account: Option<i64>,
    symbol: Option<String>,
    side: Option<Side>,
    last_qty: Option<i64>,
    last_px: Option<f64>,
    underlying_last_px: Option<f64>,
    last_mkt: Option<String>,
    last_liquidity_ind: Option<String>,
    simple_order_qty: Option<f64>,
    order_qty: Option<i64>,
    price: Option<f64>,
    display_qty: Option<i64>,
    stop_px: Option<f64>,
    peg_offset_value: Option<f64>,
    peg_price_type: Option<PegPriceType>,
    currency: Option<String>,
    settl_currency: Option<String>,
    exec_type: Option<String>,
    ord_type: Option<OrdType>,
    time_in_force: Option<TimeInForce>,
    exec_inst: Option<ExecInst>,
    contingency_type: Option<ContingencyType>,
    ex_destination: Option<String>,
    ord_status: Option<String>,
    triggered: Option<String>,
    working_indicator: Option<bool>,
    ord_rej_reason: Option<String>,
    simple_leaves_qty: Option<f64>,
    leaves_qty: Option<i64>,
    simple_cum_qty: Option<f64>,
    cum_qty: Option<i64>,
    avg_px: Option<f64>,
    commission: Option<f64>,
    trade_publish_indicator: Option<String>,
    multi_leg_reporting_type: Option<String>,
    text: Option<String>,
    #[serde(rename = "trdMatchID")]
    trd_match_id: Option<Uuid>,
    exec_cost: Option<i64>,
    exec_comm: Option<i64>,
    home_notional: Option<f64>,
    foreign_notional: Option<f64>,
    transact_time: Option<DateTime<Utc>>,
    timestamp: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Swap Funding History
pub struct Funding {
    timestamp: DateTime<Utc>,
    symbol: String,
    funding_interval: Option<DateTime<Utc>>,
    funding_rate: Option<f64>,
    funding_rate_daily: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Tradeable Contracts, Indices, and History
pub struct Instrument {
    symbol: String,
    root_symbol: Option<String>,
    state: Option<String>,
    typ: Option<String>,
    listing: Option<DateTime<Utc>>,
    front: Option<DateTime<Utc>>,
    expiry: Option<DateTime<Utc>>,
    settle: Option<DateTime<Utc>>,
    relist_interval: Option<DateTime<Utc>>,
    inverse_leg: Option<String>,
    sell_leg: Option<String>,
    buy_leg: Option<String>,
    option_strike_pcnt: Option<f64>,
    option_strike_round: Option<f64>,
    option_strike_price: Option<f64>,
    option_multiplier: Option<f64>,
    position_currency: Option<String>,
    underlying: Option<String>,
    quote_currency: Option<String>,
    underlying_symbol: Option<String>,
    reference: Option<String>,
    reference_symbol: Option<String>,
    calc_interval: Option<DateTime<Utc>>,
    publish_interval: Option<DateTime<Utc>>,
    publish_time: Option<DateTime<Utc>>,
    max_order_qty: Option<i64>,
    max_price: Option<f64>,
    lot_size: Option<i64>,
    tick_size: Option<f64>,
    multiplier: Option<i64>,
    settl_currency: Option<String>,
    underlying_to_position_multiplier: Option<i64>,
    underlying_to_settle_multiplier: Option<i64>,
    quote_to_settle_multiplier: Option<i64>,
    is_quanto: Option<bool>,
    is_inverse: Option<bool>,
    init_margin: Option<f64>,
    maint_margin: Option<f64>,
    risk_limit: Option<i64>,
    risk_step: Option<i64>,
    limit: Option<f64>,
    capped: Option<bool>,
    taxed: Option<bool>,
    deleverage: Option<bool>,
    maker_fee: Option<f64>,
    taker_fee: Option<f64>,
    settlement_fee: Option<f64>,
    insurance_fee: Option<f64>,
    funding_base_symbol: Option<String>,
    funding_quote_symbol: Option<String>,
    funding_premium_symbol: Option<String>,
    funding_timestamp: Option<DateTime<Utc>>,
    funding_interval: Option<DateTime<Utc>>,
    funding_rate: Option<f64>,
    indicative_funding_rate: Option<f64>,
    rebalance_timestamp: Option<DateTime<Utc>>,
    rebalance_interval: Option<DateTime<Utc>>,
    opening_timestamp: Option<DateTime<Utc>>,
    closing_timestamp: Option<DateTime<Utc>>,
    session_interval: Option<DateTime<Utc>>,
    prev_close_price: Option<f64>,
    limit_down_price: Option<f64>,
    limit_up_price: Option<f64>,
    bankrupt_limit_down_price: Option<f64>,
    bankrupt_limit_up_price: Option<f64>,
    prev_total_volume: Option<i64>,
    total_volume: Option<i64>,
    volume: Option<i64>,
    volume24h: Option<i64>,
    prev_total_turnover: Option<i64>,
    total_turnover: Option<i64>,
    turnover: Option<i64>,
    turnover24h: Option<i64>,
    home_notional24h: Option<f64>,
    foreign_notional24h: Option<f64>,
    prev_price24h: Option<f64>,
    vwap: Option<f64>,
    high_price: Option<f64>,
    low_price: Option<f64>,
    last_price: Option<f64>,
    last_price_protected: Option<f64>,
    last_tick_direction: Option<String>,
    last_change_pcnt: Option<f64>,
    bid_price: Option<f64>,
    mid_price: Option<f64>,
    ask_price: Option<f64>,
    impact_bid_price: Option<f64>,
    impact_mid_price: Option<f64>,
    impact_ask_price: Option<f64>,
    has_liquidity: Option<bool>,
    open_interest: Option<i64>,
    open_value: Option<i64>,
    fair_method: Option<String>,
    fair_basis_rate: Option<f64>,
    fair_basis: Option<f64>,
    fair_price: Option<f64>,
    mark_method: Option<String>,
    mark_price: Option<f64>,
    indicative_tax_rate: Option<f64>,
    indicative_settle_price: Option<f64>,
    option_underlying_price: Option<f64>,
    settled_price: Option<f64>,
    timestamp: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct InstrumentInterval {
    intervals: Vec<String>,
    symbols: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct IndexComposite {
    timestamp: DateTime<Utc>,
    symbol: Option<String>,
    index_symbol: Option<String>,
    reference: Option<String>,
    last_price: Option<f64>,
    weight: Option<f64>,
    logged: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Insurance Fund Data
pub struct Insurance {
    currency: String,
    timestamp: DateTime<Utc>,
    wallet_balance: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Information on Top Users
pub struct Leaderboard {
    name: String,
    is_real_name: Option<bool>,
    profit: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Active Liquidations
pub struct Liquidation {
    #[serde(rename = "orderID")]
    order_id: Uuid,
    symbol: Option<String>,
    side: Option<Side>,
    price: Option<f64>,
    leaves_qty: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Account Notifications
pub struct GlobalNotification {
    id: Option<i32>,
    date: DateTime<Utc>,
    title: String,
    body: String,
    ttl: i32,
    r#type: Option<String>,
    closable: Option<bool>,
    persist: Option<bool>,
    wait_for_visibility: Option<bool>,
    sound: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Placement, Cancellation, Amending, and History
pub struct Order {
    #[serde(rename = "orderID")]
    order_id: Uuid,
    #[serde(rename = "clOrdID")]
    cl_ord_id: Option<String>,
    #[serde(rename = "clOrdLinkID")]
    cl_ord_link_id: Option<String>,
    account: Option<i64>,
    symbol: Option<String>,
    side: Option<Side>,
    simple_order_qty: Option<f64>,
    order_qty: Option<i64>,
    price: Option<f64>,
    display_qty: Option<i64>,
    stop_px: Option<f64>,
    peg_offset_value: Option<f64>,
    peg_price_type: Option<PegPriceType>,
    currency: Option<String>,
    settl_currency: Option<String>,
    ord_type: Option<OrdType>,
    time_in_force: Option<TimeInForce>,
    exec_inst: Option<ExecInst>,
    contingency_type: Option<ContingencyType>,
    ex_destination: Option<String>,
    ord_status: Option<String>,
    triggered: Option<String>,
    working_indicator: Option<bool>,
    ord_rej_reason: Option<String>,
    simple_leaves_qty: Option<f64>,
    leaves_qty: Option<i64>,
    simple_cum_qty: Option<f64>,
    cum_qty: Option<i64>,
    avg_px: Option<f64>,
    multi_leg_reporting_type: Option<String>,
    text: Option<String>,
    transact_time: Option<DateTime<Utc>>,
    timestamp: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct OrderBookL2 {
    symbol: String,
    id: i64,
    side: Side,
    size: Option<i64>,
    price: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Summary of Open and Closed Positions
pub struct Position {
    account: i64,
    symbol: String,
    currency: String,
    underlying: Option<String>,
    quote_currency: Option<String>,
    commission: Option<f64>,
    init_margin_req: Option<f64>,
    maint_margin_req: Option<f64>,
    risk_limit: Option<i64>,
    leverage: Option<f64>,
    cross_margin: Option<bool>,
    deleverage_percentile: Option<f64>,
    rebalanced_pnl: Option<i64>,
    prev_realised_pnl: Option<i64>,
    prev_unrealised_pnl: Option<i64>,
    prev_close_price: Option<f64>,
    opening_timestamp: Option<DateTime<Utc>>,
    opening_qty: Option<i64>,
    opening_cost: Option<i64>,
    opening_comm: Option<i64>,
    open_order_buy_qty: Option<i64>,
    open_order_buy_cost: Option<i64>,
    open_order_buy_premium: Option<i64>,
    open_order_sell_qty: Option<i64>,
    open_order_sell_cost: Option<i64>,
    open_order_sell_premium: Option<i64>,
    exec_buy_qty: Option<i64>,
    exec_buy_cost: Option<i64>,
    exec_sell_qty: Option<i64>,
    exec_sell_cost: Option<i64>,
    exec_qty: Option<i64>,
    exec_cost: Option<i64>,
    exec_comm: Option<i64>,
    current_timestamp: Option<DateTime<Utc>>,
    current_qty: Option<i64>,
    current_cost: Option<i64>,
    current_comm: Option<i64>,
    realised_cost: Option<i64>,
    unrealised_cost: Option<i64>,
    gross_open_cost: Option<i64>,
    gross_open_premium: Option<i64>,
    gross_exec_cost: Option<i64>,
    is_open: Option<bool>,
    mark_price: Option<f64>,
    mark_value: Option<i64>,
    risk_value: Option<i64>,
    home_notional: Option<f64>,
    foreign_notional: Option<f64>,
    pos_state: Option<String>,
    pos_cost: Option<i64>,
    pos_cost2: Option<i64>,
    pos_cross: Option<i64>,
    pos_init: Option<i64>,
    pos_comm: Option<i64>,
    pos_loss: Option<i64>,
    pos_margin: Option<i64>,
    pos_maint: Option<i64>,
    pos_allowance: Option<i64>,
    taxable_margin: Option<i64>,
    init_margin: Option<i64>,
    maint_margin: Option<i64>,
    session_margin: Option<i64>,
    target_excess_margin: Option<i64>,
    var_margin: Option<i64>,
    realised_gross_pnl: Option<i64>,
    realised_tax: Option<i64>,
    realised_pnl: Option<i64>,
    unrealised_gross_pnl: Option<i64>,
    long_bankrupt: Option<i64>,
    short_bankrupt: Option<i64>,
    tax_base: Option<i64>,
    indicative_tax_rate: Option<f64>,
    indicative_tax: Option<i64>,
    unrealised_tax: Option<i64>,
    unrealised_pnl: Option<i64>,
    unrealised_pnl_pcnt: Option<f64>,
    unrealised_roe_pcnt: Option<f64>,
    simple_qty: Option<f64>,
    simple_cost: Option<f64>,
    simple_value: Option<f64>,
    simple_pnl: Option<f64>,
    simple_pnl_pcnt: Option<f64>,
    avg_cost_price: Option<f64>,
    avg_entry_price: Option<f64>,
    break_even_price: Option<f64>,
    margin_call_price: Option<f64>,
    liquidation_price: Option<f64>,
    bankrupt_price: Option<f64>,
    timestamp: Option<DateTime<Utc>>,
    last_price: Option<f64>,
    last_value: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Best Bid/Offer Snapshots & Historical Bins
pub struct Quote {
    timestamp: DateTime<Utc>,
    symbol: String,
    bid_size: Option<i64>,
    bid_price: Option<f64>,
    ask_price: Option<f64>,
    ask_size: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Historical Settlement Data
pub struct Settlement {
    timestamp: DateTime<Utc>,
    symbol: String,
    settlement_type: Option<String>,
    settled_price: Option<f64>,
    option_strike_price: Option<f64>,
    option_underlying_price: Option<f64>,
    bankrupt: Option<i64>,
    tax_base: Option<i64>,
    tax_rate: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Exchange Statistics
pub struct Stats {
    root_symbol: String,
    currency: Option<String>,
    volume24h: Option<i64>,
    turnover24h: Option<i64>,
    open_interest: Option<i64>,
    open_value: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct StatsHistory {
    date: DateTime<Utc>,
    root_symbol: String,
    currency: Option<String>,
    volume: Option<i64>,
    turnover: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct StatsUSD {
    root_symbol: String,
    currency: Option<String>,
    turnover24h: Option<i64>,
    turnover30d: Option<i64>,
    turnover365d: Option<i64>,
    turnover: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Individual & Bucketed Trades
pub struct Trade {
    timestamp: DateTime<Utc>,
    symbol: String,
    side: Option<Side>,
    size: Option<i64>,
    price: Option<f64>,
    tick_direction: Option<String>,
    #[serde(rename = "trdMatchID")]
    trd_match_id: Option<Uuid>,
    gross_value: Option<i64>,
    home_notional: Option<f64>,
    foreign_notional: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct TradeBin {
    timestamp: DateTime<Utc>,
    symbol: String,
    open: Option<f64>,
    high: Option<f64>,
    low: Option<f64>,
    close: Option<f64>,
    trades: Option<i64>,
    volume: Option<i64>,
    vwap: Option<f64>,
    last_size: Option<i64>,
    turnover: Option<i64>,
    home_notional: Option<f64>,
    foreign_notional: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct Wallet {
    account: i64,
    currency: String,
    prev_deposited: Option<i64>,
    prev_withdrawn: Option<i64>,
    prev_transfer_in: Option<i64>,
    prev_transfer_out: Option<i64>,
    prev_amount: Option<i64>,
    prev_timestamp: Option<DateTime<Utc>>,
    delta_deposited: Option<i64>,
    delta_withdrawn: Option<i64>,
    delta_transfer_in: Option<i64>,
    delta_transfer_out: Option<i64>,
    delta_amount: Option<i64>,
    deposited: Option<i64>,
    withdrawn: Option<i64>,
    transfer_in: Option<i64>,
    transfer_out: Option<i64>,
    amount: Option<i64>,
    pending_credit: Option<i64>,
    pending_debit: Option<i64>,
    confirmed_debit: Option<i64>,
    timestamp: Option<DateTime<Utc>>,
    addr: Option<String>,
    script: Option<String>,
    withdrawal_lock: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct Transaction {
    #[serde(rename = "transactID")]
    transact_id: Uuid,
    account: Option<i64>,
    currency: Option<String>,
    transact_type: Option<String>,
    amount: Option<i64>,
    fee: Option<i64>,
    transact_status: Option<String>,
    address: Option<String>,
    tx: Option<String>,
    text: Option<String>,
    transact_time: Option<DateTime<Utc>>,
    timestamp: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct AccessToken {
    id: String,
    ttl: Option<f64>,
    created: Option<DateTime<Utc>>,
    user_id: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct Affiliate {
    account: i64,
    currency: String,
    prev_payout: Option<i64>,
    prev_turnover: Option<i64>,
    prev_comm: Option<i64>,
    prev_timestamp: Option<DateTime<Utc>>,
    exec_turnover: Option<i64>,
    exec_comm: Option<i64>,
    total_referrals: Option<i64>,
    total_turnover: Option<i64>,
    total_comm: Option<i64>,
    payout_pcnt: Option<f64>,
    pending_payout: Option<i64>,
    timestamp: Option<DateTime<Utc>>,
    referrer_account: Option<f64>,
    referral_discount: Option<f64>,
    affiliate_payout: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Account Operations
pub struct User {
    id: Option<i32>,
    owner_id: Option<i32>,
    firstname: Option<String>,
    lastname: Option<String>,
    username: String,
    email: String,
    phone: Option<String>,
    created: Option<DateTime<Utc>>,
    last_updated: Option<DateTime<Utc>>,
    preferences: Option<UserPreferences>,
    restricted_engine_fields: Option<Value>,
    tfa_enabled: Option<String>,
    #[serde(rename = "affiliateID")]
    affiliate_id: Option<String>,
    pgp_pub_key: Option<String>,
    country: Option<String>,
    geoip_country: Option<String>,
    geoip_region: Option<String>,
    typ: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct UserCommission {
    maker_fee: Option<f64>,
    taker_fee: Option<f64>,
    settlement_fee: Option<f64>,
    max_fee: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct Margin {
    account: i64,
    currency: String,
    risk_limit: Option<i64>,
    prev_state: Option<String>,
    state: Option<String>,
    action: Option<String>,
    amount: Option<i64>,
    pending_credit: Option<i64>,
    pending_debit: Option<i64>,
    confirmed_debit: Option<i64>,
    prev_realised_pnl: Option<i64>,
    prev_unrealised_pnl: Option<i64>,
    gross_comm: Option<i64>,
    gross_open_cost: Option<i64>,
    gross_open_premium: Option<i64>,
    gross_exec_cost: Option<i64>,
    gross_mark_value: Option<i64>,
    risk_value: Option<i64>,
    taxable_margin: Option<i64>,
    init_margin: Option<i64>,
    maint_margin: Option<i64>,
    session_margin: Option<i64>,
    target_excess_margin: Option<i64>,
    var_margin: Option<i64>,
    realised_pnl: Option<i64>,
    unrealised_pnl: Option<i64>,
    indicative_tax: Option<i64>,
    unrealised_profit: Option<i64>,
    synthetic_margin: Option<i64>,
    wallet_balance: Option<i64>,
    margin_balance: Option<i64>,
    margin_balance_pcnt: Option<f64>,
    margin_leverage: Option<f64>,
    margin_used_pcnt: Option<f64>,
    excess_margin: Option<i64>,
    excess_margin_pcnt: Option<f64>,
    available_margin: Option<i64>,
    withdrawable_margin: Option<i64>,
    timestamp: Option<DateTime<Utc>>,
    gross_last_value: Option<i64>,
    commission: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// User communication SNS token
pub struct CommunicationToken {
    id: String,
    user_id: i32,
    device_token: String,
    channel: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// User Events for auditing
pub struct UserEvent {
    id: Option<f64>,
    r#type: String,
    status: String,
    user_id: f64,
    created_by_id: f64,
    ip: Option<String>,
    geoip_country: Option<String>,
    geoip_region: Option<String>,
    geoip_sub_region: Option<String>,
    event_meta: Option<Value>,
    created: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// empty
pub struct UserPreferences {
    alert_on_liquidations: Option<bool>,
    animations_enabled: Option<bool>,
    announcements_last_seen: Option<DateTime<Utc>>,
    #[serde(rename = "chatChannelID")]
    chat_channel_id: Option<f64>,
    color_theme: Option<String>,
    currency: Option<String>,
    debug: Option<bool>,
    disable_emails: Option<Vec<String>>,
    disable_push: Option<Vec<String>>,
    hide_confirm_dialogs: Option<Vec<String>>,
    hide_connection_modal: Option<bool>,
    hide_from_leaderboard: Option<bool>,
    hide_name_from_leaderboard: Option<bool>,
    hide_notifications: Option<Vec<String>>,
    locale: Option<String>,
    msgs_seen: Option<Vec<String>>,
    order_book_binning: Option<Value>,
    order_book_type: Option<String>,
    order_clear_immediate: Option<bool>,
    order_controls_plus_minus: Option<bool>,
    show_locale_numbers: Option<bool>,
    sounds: Option<Vec<String>>,
    strict_ip_check: Option<bool>,
    strict_timeout: Option<bool>,
    ticker_group: Option<String>,
    ticker_pinned: Option<bool>,
    trade_layout: Option<String>,
}
