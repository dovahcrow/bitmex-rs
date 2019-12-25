use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
/// Public Announcements
pub struct Announcement {
    pub id: i32,
    pub link: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub date: Option<DateTime<Utc>>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Error {
    pub error: ErrorError
}
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct ErrorError {
    pub message: Option<String>,
    pub name: Option<String>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
/// Persistent API Keys for Developers
pub struct APIKey {
    pub id: String,
    pub secret: Option<String>,
    pub name: String,
    pub nonce: i64,
    pub cidr: Option<String>,
    pub permissions: Vec<Value>,
    pub enabled: Option<bool>,
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub created: Option<DateTime<Utc>>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
/// Trollbox Data
pub struct Chat {
    pub id: Option<i32>,
    pub date: DateTime<Utc>,
    pub user: String,
    pub message: String,
    pub html: String,
    #[serde(rename = "fromBot")]
    pub from_bot: Option<bool>,
    #[serde(rename = "channelID")]
    pub channel_id: Option<f64>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatChannel {
    pub id: Option<i32>,
    pub name: String
}
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct ConnectedUsers {
    pub users: Option<i32>,
    pub bots: Option<i32>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
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
    pub side: Option<super::Side>,
    #[serde(rename = "lastQty")]
    pub last_qty: Option<i64>,
    #[serde(rename = "lastPx")]
    pub last_px: Option<f64>,
    #[serde(rename = "underlyingLastPx")]
    pub underlying_last_px: Option<f64>,
    #[serde(rename = "lastMkt")]
    pub last_mkt: Option<String>,
    #[serde(rename = "lastLiquidityInd")]
    pub last_liquidity_ind: Option<String>,
    #[serde(rename = "simpleOrderQty")]
    pub simple_order_qty: Option<f64>,
    #[serde(rename = "orderQty")]
    pub order_qty: Option<i64>,
    pub price: Option<f64>,
    #[serde(rename = "displayQty")]
    pub display_qty: Option<i64>,
    #[serde(rename = "stopPx")]
    pub stop_px: Option<f64>,
    #[serde(rename = "pegOffsetValue")]
    pub peg_offset_value: Option<f64>,
    #[serde(rename = "pegPriceType")]
    pub peg_price_type: Option<super::PegPriceType>,
    pub currency: Option<String>,
    #[serde(rename = "settlCurrency")]
    pub settl_currency: Option<String>,
    #[serde(rename = "execType")]
    pub exec_type: Option<String>,
    #[serde(rename = "ordType")]
    pub ord_type: Option<super::OrdType>,
    #[serde(rename = "timeInForce")]
    pub time_in_force: Option<super::TimeInForce>,
    #[serde(rename = "execInst")]
    pub exec_inst: Option<super::ExecInst>,
    #[serde(rename = "contingencyType")]
    pub contingency_type: Option<super::ContingencyType>,
    #[serde(rename = "exDestination")]
    pub ex_destination: Option<String>,
    #[serde(rename = "ordStatus")]
    pub ord_status: Option<String>,
    pub triggered: Option<String>,
    #[serde(rename = "workingIndicator")]
    pub working_indicator: Option<bool>,
    #[serde(rename = "ordRejReason")]
    pub ord_rej_reason: Option<String>,
    #[serde(rename = "simpleLeavesQty")]
    pub simple_leaves_qty: Option<f64>,
    #[serde(rename = "leavesQty")]
    pub leaves_qty: Option<i64>,
    #[serde(rename = "simpleCumQty")]
    pub simple_cum_qty: Option<f64>,
    #[serde(rename = "cumQty")]
    pub cum_qty: Option<i64>,
    #[serde(rename = "avgPx")]
    pub avg_px: Option<f64>,
    pub commission: Option<f64>,
    #[serde(rename = "tradePublishIndicator")]
    pub trade_publish_indicator: Option<String>,
    #[serde(rename = "multiLegReportingType")]
    pub multi_leg_reporting_type: Option<String>,
    pub text: Option<String>,
    #[serde(rename = "trdMatchID")]
    pub trd_match_id: Option<Uuid>,
    #[serde(rename = "execCost")]
    pub exec_cost: Option<i64>,
    #[serde(rename = "execComm")]
    pub exec_comm: Option<i64>,
    #[serde(rename = "homeNotional")]
    pub home_notional: Option<f64>,
    #[serde(rename = "foreignNotional")]
    pub foreign_notional: Option<f64>,
    #[serde(rename = "transactTime")]
    pub transact_time: Option<DateTime<Utc>>,
    pub timestamp: Option<DateTime<Utc>>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
/// Swap Funding History
pub struct Funding {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    #[serde(rename = "fundingInterval")]
    pub funding_interval: Option<DateTime<Utc>>,
    #[serde(rename = "fundingRate")]
    pub funding_rate: Option<f64>,
    #[serde(rename = "fundingRateDaily")]
    pub funding_rate_daily: Option<f64>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
/// Tradeable Contracts, Indices, and History
pub struct Instrument {
    pub symbol: String,
    #[serde(rename = "rootSymbol")]
    pub root_symbol: Option<String>,
    pub state: Option<String>,
    pub typ: Option<String>,
    pub listing: Option<DateTime<Utc>>,
    pub front: Option<DateTime<Utc>>,
    pub expiry: Option<DateTime<Utc>>,
    pub settle: Option<DateTime<Utc>>,
    #[serde(rename = "relistInterval")]
    pub relist_interval: Option<DateTime<Utc>>,
    #[serde(rename = "inverseLeg")]
    pub inverse_leg: Option<String>,
    #[serde(rename = "sellLeg")]
    pub sell_leg: Option<String>,
    #[serde(rename = "buyLeg")]
    pub buy_leg: Option<String>,
    #[serde(rename = "optionStrikePcnt")]
    pub option_strike_pcnt: Option<f64>,
    #[serde(rename = "optionStrikeRound")]
    pub option_strike_round: Option<f64>,
    #[serde(rename = "optionStrikePrice")]
    pub option_strike_price: Option<f64>,
    #[serde(rename = "optionMultiplier")]
    pub option_multiplier: Option<f64>,
    #[serde(rename = "positionCurrency")]
    pub position_currency: Option<String>,
    pub underlying: Option<String>,
    #[serde(rename = "quoteCurrency")]
    pub quote_currency: Option<String>,
    #[serde(rename = "underlyingSymbol")]
    pub underlying_symbol: Option<String>,
    pub reference: Option<String>,
    #[serde(rename = "referenceSymbol")]
    pub reference_symbol: Option<String>,
    #[serde(rename = "calcInterval")]
    pub calc_interval: Option<DateTime<Utc>>,
    #[serde(rename = "publishInterval")]
    pub publish_interval: Option<DateTime<Utc>>,
    #[serde(rename = "publishTime")]
    pub publish_time: Option<DateTime<Utc>>,
    #[serde(rename = "maxOrderQty")]
    pub max_order_qty: Option<i64>,
    #[serde(rename = "maxPrice")]
    pub max_price: Option<f64>,
    #[serde(rename = "lotSize")]
    pub lot_size: Option<i64>,
    #[serde(rename = "tickSize")]
    pub tick_size: Option<f64>,
    pub multiplier: Option<i64>,
    #[serde(rename = "settlCurrency")]
    pub settl_currency: Option<String>,
    #[serde(rename = "underlyingToPositionMultiplier")]
    pub underlying_to_position_multiplier: Option<i64>,
    #[serde(rename = "underlyingToSettleMultiplier")]
    pub underlying_to_settle_multiplier: Option<i64>,
    #[serde(rename = "quoteToSettleMultiplier")]
    pub quote_to_settle_multiplier: Option<i64>,
    #[serde(rename = "isQuanto")]
    pub is_quanto: Option<bool>,
    #[serde(rename = "isInverse")]
    pub is_inverse: Option<bool>,
    #[serde(rename = "initMargin")]
    pub init_margin: Option<f64>,
    #[serde(rename = "maintMargin")]
    pub maint_margin: Option<f64>,
    #[serde(rename = "riskLimit")]
    pub risk_limit: Option<i64>,
    #[serde(rename = "riskStep")]
    pub risk_step: Option<i64>,
    pub limit: Option<f64>,
    pub capped: Option<bool>,
    pub taxed: Option<bool>,
    pub deleverage: Option<bool>,
    #[serde(rename = "makerFee")]
    pub maker_fee: Option<f64>,
    #[serde(rename = "takerFee")]
    pub taker_fee: Option<f64>,
    #[serde(rename = "settlementFee")]
    pub settlement_fee: Option<f64>,
    #[serde(rename = "insuranceFee")]
    pub insurance_fee: Option<f64>,
    #[serde(rename = "fundingBaseSymbol")]
    pub funding_base_symbol: Option<String>,
    #[serde(rename = "fundingQuoteSymbol")]
    pub funding_quote_symbol: Option<String>,
    #[serde(rename = "fundingPremiumSymbol")]
    pub funding_premium_symbol: Option<String>,
    #[serde(rename = "fundingTimestamp")]
    pub funding_timestamp: Option<DateTime<Utc>>,
    #[serde(rename = "fundingInterval")]
    pub funding_interval: Option<DateTime<Utc>>,
    #[serde(rename = "fundingRate")]
    pub funding_rate: Option<f64>,
    #[serde(rename = "indicativeFundingRate")]
    pub indicative_funding_rate: Option<f64>,
    #[serde(rename = "rebalanceTimestamp")]
    pub rebalance_timestamp: Option<DateTime<Utc>>,
    #[serde(rename = "rebalanceInterval")]
    pub rebalance_interval: Option<DateTime<Utc>>,
    #[serde(rename = "openingTimestamp")]
    pub opening_timestamp: Option<DateTime<Utc>>,
    #[serde(rename = "closingTimestamp")]
    pub closing_timestamp: Option<DateTime<Utc>>,
    #[serde(rename = "sessionInterval")]
    pub session_interval: Option<DateTime<Utc>>,
    #[serde(rename = "prevClosePrice")]
    pub prev_close_price: Option<f64>,
    #[serde(rename = "limitDownPrice")]
    pub limit_down_price: Option<f64>,
    #[serde(rename = "limitUpPrice")]
    pub limit_up_price: Option<f64>,
    #[serde(rename = "bankruptLimitDownPrice")]
    pub bankrupt_limit_down_price: Option<f64>,
    #[serde(rename = "bankruptLimitUpPrice")]
    pub bankrupt_limit_up_price: Option<f64>,
    #[serde(rename = "prevTotalVolume")]
    pub prev_total_volume: Option<i64>,
    #[serde(rename = "totalVolume")]
    pub total_volume: Option<i64>,
    pub volume: Option<i64>,
    pub volume24h: Option<i64>,
    #[serde(rename = "prevTotalTurnover")]
    pub prev_total_turnover: Option<i64>,
    #[serde(rename = "totalTurnover")]
    pub total_turnover: Option<i64>,
    pub turnover: Option<i64>,
    pub turnover24h: Option<i64>,
    #[serde(rename = "homeNotional24h")]
    pub home_notional24h: Option<f64>,
    #[serde(rename = "foreignNotional24h")]
    pub foreign_notional24h: Option<f64>,
    #[serde(rename = "prevPrice24h")]
    pub prev_price24h: Option<f64>,
    pub vwap: Option<f64>,
    #[serde(rename = "highPrice")]
    pub high_price: Option<f64>,
    #[serde(rename = "lowPrice")]
    pub low_price: Option<f64>,
    #[serde(rename = "lastPrice")]
    pub last_price: Option<f64>,
    #[serde(rename = "lastPriceProtected")]
    pub last_price_protected: Option<f64>,
    #[serde(rename = "lastTickDirection")]
    pub last_tick_direction: Option<String>,
    #[serde(rename = "lastChangePcnt")]
    pub last_change_pcnt: Option<f64>,
    #[serde(rename = "bidPrice")]
    pub bid_price: Option<f64>,
    #[serde(rename = "midPrice")]
    pub mid_price: Option<f64>,
    #[serde(rename = "askPrice")]
    pub ask_price: Option<f64>,
    #[serde(rename = "impactBidPrice")]
    pub impact_bid_price: Option<f64>,
    #[serde(rename = "impactMidPrice")]
    pub impact_mid_price: Option<f64>,
    #[serde(rename = "impactAskPrice")]
    pub impact_ask_price: Option<f64>,
    #[serde(rename = "hasLiquidity")]
    pub has_liquidity: Option<bool>,
    #[serde(rename = "openInterest")]
    pub open_interest: Option<i64>,
    #[serde(rename = "openValue")]
    pub open_value: Option<i64>,
    #[serde(rename = "fairMethod")]
    pub fair_method: Option<String>,
    #[serde(rename = "fairBasisRate")]
    pub fair_basis_rate: Option<f64>,
    #[serde(rename = "fairBasis")]
    pub fair_basis: Option<f64>,
    #[serde(rename = "fairPrice")]
    pub fair_price: Option<f64>,
    #[serde(rename = "markMethod")]
    pub mark_method: Option<String>,
    #[serde(rename = "markPrice")]
    pub mark_price: Option<f64>,
    #[serde(rename = "indicativeTaxRate")]
    pub indicative_tax_rate: Option<f64>,
    #[serde(rename = "indicativeSettlePrice")]
    pub indicative_settle_price: Option<f64>,
    #[serde(rename = "optionUnderlyingPrice")]
    pub option_underlying_price: Option<f64>,
    #[serde(rename = "settledPrice")]
    pub settled_price: Option<f64>,
    pub timestamp: Option<DateTime<Utc>>
}
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct InstrumentInterval {
    pub intervals: Vec<String>,
    pub symbols: Vec<String>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndexComposite {
    pub timestamp: DateTime<Utc>,
    pub symbol: Option<String>,
    #[serde(rename = "indexSymbol")]
    pub index_symbol: Option<String>,
    pub reference: Option<String>,
    #[serde(rename = "lastPrice")]
    pub last_price: Option<f64>,
    pub weight: Option<f64>,
    pub logged: Option<DateTime<Utc>>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
/// Insurance Fund Data
pub struct Insurance {
    pub currency: String,
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "walletBalance")]
    pub wallet_balance: Option<i64>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
/// Information on Top Users
pub struct Leaderboard {
    pub name: String,
    #[serde(rename = "isRealName")]
    pub is_real_name: Option<bool>,
    pub profit: Option<f64>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
/// Active Liquidations
pub struct Liquidation {
    #[serde(rename = "orderID")]
    pub order_id: Uuid,
    pub symbol: Option<String>,
    pub side: Option<super::Side>,
    pub price: Option<f64>,
    #[serde(rename = "leavesQty")]
    pub leaves_qty: Option<i64>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
/// Account Notifications
pub struct GlobalNotification {
    pub id: Option<i32>,
    pub date: DateTime<Utc>,
    pub title: String,
    pub body: String,
    pub ttl: i32,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub closable: Option<bool>,
    pub persist: Option<bool>,
    #[serde(rename = "waitForVisibility")]
    pub wait_for_visibility: Option<bool>,
    pub sound: Option<String>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
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
    pub side: Option<super::Side>,
    #[serde(rename = "simpleOrderQty")]
    pub simple_order_qty: Option<f64>,
    #[serde(rename = "orderQty")]
    pub order_qty: Option<i64>,
    pub price: Option<f64>,
    #[serde(rename = "displayQty")]
    pub display_qty: Option<i64>,
    #[serde(rename = "stopPx")]
    pub stop_px: Option<f64>,
    #[serde(rename = "pegOffsetValue")]
    pub peg_offset_value: Option<f64>,
    #[serde(rename = "pegPriceType")]
    pub peg_price_type: Option<super::PegPriceType>,
    pub currency: Option<String>,
    #[serde(rename = "settlCurrency")]
    pub settl_currency: Option<String>,
    #[serde(rename = "ordType")]
    pub ord_type: Option<super::OrdType>,
    #[serde(rename = "timeInForce")]
    pub time_in_force: Option<super::TimeInForce>,
    #[serde(rename = "execInst")]
    pub exec_inst: Option<super::ExecInst>,
    #[serde(rename = "contingencyType")]
    pub contingency_type: Option<super::ContingencyType>,
    #[serde(rename = "exDestination")]
    pub ex_destination: Option<String>,
    #[serde(rename = "ordStatus")]
    pub ord_status: Option<String>,
    pub triggered: Option<String>,
    #[serde(rename = "workingIndicator")]
    pub working_indicator: Option<bool>,
    #[serde(rename = "ordRejReason")]
    pub ord_rej_reason: Option<String>,
    #[serde(rename = "simpleLeavesQty")]
    pub simple_leaves_qty: Option<f64>,
    #[serde(rename = "leavesQty")]
    pub leaves_qty: Option<i64>,
    #[serde(rename = "simpleCumQty")]
    pub simple_cum_qty: Option<f64>,
    #[serde(rename = "cumQty")]
    pub cum_qty: Option<i64>,
    #[serde(rename = "avgPx")]
    pub avg_px: Option<f64>,
    #[serde(rename = "multiLegReportingType")]
    pub multi_leg_reporting_type: Option<String>,
    pub text: Option<String>,
    #[serde(rename = "transactTime")]
    pub transact_time: Option<DateTime<Utc>>,
    pub timestamp: Option<DateTime<Utc>>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderBookL2 {
    pub symbol: String,
    pub id: i64,
    pub side: super::Side,
    pub size: Option<i64>,
    pub price: Option<f64>
}
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Ny(serde_json::Value);
#[derive(Clone, Debug, Deserialize, Serialize)]
/// Summary of Open and Closed Positions
pub struct Position {
    pub account: i64,
    pub symbol: String,
    pub currency: String,
    pub underlying: Option<String>,
    #[serde(rename = "quoteCurrency")]
    pub quote_currency: Option<String>,
    pub commission: Option<f64>,
    #[serde(rename = "initMarginReq")]
    pub init_margin_req: Option<f64>,
    #[serde(rename = "maintMarginReq")]
    pub maint_margin_req: Option<f64>,
    #[serde(rename = "riskLimit")]
    pub risk_limit: Option<i64>,
    pub leverage: Option<f64>,
    #[serde(rename = "crossMargin")]
    pub cross_margin: Option<bool>,
    #[serde(rename = "deleveragePercentile")]
    pub deleverage_percentile: Option<f64>,
    #[serde(rename = "rebalancedPnl")]
    pub rebalanced_pnl: Option<i64>,
    #[serde(rename = "prevRealisedPnl")]
    pub prev_realised_pnl: Option<i64>,
    #[serde(rename = "prevUnrealisedPnl")]
    pub prev_unrealised_pnl: Option<i64>,
    #[serde(rename = "prevClosePrice")]
    pub prev_close_price: Option<f64>,
    #[serde(rename = "openingTimestamp")]
    pub opening_timestamp: Option<DateTime<Utc>>,
    #[serde(rename = "openingQty")]
    pub opening_qty: Option<i64>,
    #[serde(rename = "openingCost")]
    pub opening_cost: Option<i64>,
    #[serde(rename = "openingComm")]
    pub opening_comm: Option<i64>,
    #[serde(rename = "openOrderBuyQty")]
    pub open_order_buy_qty: Option<i64>,
    #[serde(rename = "openOrderBuyCost")]
    pub open_order_buy_cost: Option<i64>,
    #[serde(rename = "openOrderBuyPremium")]
    pub open_order_buy_premium: Option<i64>,
    #[serde(rename = "openOrderSellQty")]
    pub open_order_sell_qty: Option<i64>,
    #[serde(rename = "openOrderSellCost")]
    pub open_order_sell_cost: Option<i64>,
    #[serde(rename = "openOrderSellPremium")]
    pub open_order_sell_premium: Option<i64>,
    #[serde(rename = "execBuyQty")]
    pub exec_buy_qty: Option<i64>,
    #[serde(rename = "execBuyCost")]
    pub exec_buy_cost: Option<i64>,
    #[serde(rename = "execSellQty")]
    pub exec_sell_qty: Option<i64>,
    #[serde(rename = "execSellCost")]
    pub exec_sell_cost: Option<i64>,
    #[serde(rename = "execQty")]
    pub exec_qty: Option<i64>,
    #[serde(rename = "execCost")]
    pub exec_cost: Option<i64>,
    #[serde(rename = "execComm")]
    pub exec_comm: Option<i64>,
    #[serde(rename = "currentTimestamp")]
    pub current_timestamp: Option<DateTime<Utc>>,
    #[serde(rename = "currentQty")]
    pub current_qty: Option<i64>,
    #[serde(rename = "currentCost")]
    pub current_cost: Option<i64>,
    #[serde(rename = "currentComm")]
    pub current_comm: Option<i64>,
    #[serde(rename = "realisedCost")]
    pub realised_cost: Option<i64>,
    #[serde(rename = "unrealisedCost")]
    pub unrealised_cost: Option<i64>,
    #[serde(rename = "grossOpenCost")]
    pub gross_open_cost: Option<i64>,
    #[serde(rename = "grossOpenPremium")]
    pub gross_open_premium: Option<i64>,
    #[serde(rename = "grossExecCost")]
    pub gross_exec_cost: Option<i64>,
    #[serde(rename = "isOpen")]
    pub is_open: Option<bool>,
    #[serde(rename = "markPrice")]
    pub mark_price: Option<f64>,
    #[serde(rename = "markValue")]
    pub mark_value: Option<i64>,
    #[serde(rename = "riskValue")]
    pub risk_value: Option<i64>,
    #[serde(rename = "homeNotional")]
    pub home_notional: Option<f64>,
    #[serde(rename = "foreignNotional")]
    pub foreign_notional: Option<f64>,
    #[serde(rename = "posState")]
    pub pos_state: Option<String>,
    #[serde(rename = "posCost")]
    pub pos_cost: Option<i64>,
    #[serde(rename = "posCost2")]
    pub pos_cost2: Option<i64>,
    #[serde(rename = "posCross")]
    pub pos_cross: Option<i64>,
    #[serde(rename = "posInit")]
    pub pos_init: Option<i64>,
    #[serde(rename = "posComm")]
    pub pos_comm: Option<i64>,
    #[serde(rename = "posLoss")]
    pub pos_loss: Option<i64>,
    #[serde(rename = "posMargin")]
    pub pos_margin: Option<i64>,
    #[serde(rename = "posMaint")]
    pub pos_maint: Option<i64>,
    #[serde(rename = "posAllowance")]
    pub pos_allowance: Option<i64>,
    #[serde(rename = "taxableMargin")]
    pub taxable_margin: Option<i64>,
    #[serde(rename = "initMargin")]
    pub init_margin: Option<i64>,
    #[serde(rename = "maintMargin")]
    pub maint_margin: Option<i64>,
    #[serde(rename = "sessionMargin")]
    pub session_margin: Option<i64>,
    #[serde(rename = "targetExcessMargin")]
    pub target_excess_margin: Option<i64>,
    #[serde(rename = "varMargin")]
    pub var_margin: Option<i64>,
    #[serde(rename = "realisedGrossPnl")]
    pub realised_gross_pnl: Option<i64>,
    #[serde(rename = "realisedTax")]
    pub realised_tax: Option<i64>,
    #[serde(rename = "realisedPnl")]
    pub realised_pnl: Option<i64>,
    #[serde(rename = "unrealisedGrossPnl")]
    pub unrealised_gross_pnl: Option<i64>,
    #[serde(rename = "longBankrupt")]
    pub long_bankrupt: Option<i64>,
    #[serde(rename = "shortBankrupt")]
    pub short_bankrupt: Option<i64>,
    #[serde(rename = "taxBase")]
    pub tax_base: Option<i64>,
    #[serde(rename = "indicativeTaxRate")]
    pub indicative_tax_rate: Option<f64>,
    #[serde(rename = "indicativeTax")]
    pub indicative_tax: Option<i64>,
    #[serde(rename = "unrealisedTax")]
    pub unrealised_tax: Option<i64>,
    #[serde(rename = "unrealisedPnl")]
    pub unrealised_pnl: Option<i64>,
    #[serde(rename = "unrealisedPnlPcnt")]
    pub unrealised_pnl_pcnt: Option<f64>,
    #[serde(rename = "unrealisedRoePcnt")]
    pub unrealised_roe_pcnt: Option<f64>,
    #[serde(rename = "simpleQty")]
    pub simple_qty: Option<f64>,
    #[serde(rename = "simpleCost")]
    pub simple_cost: Option<f64>,
    #[serde(rename = "simpleValue")]
    pub simple_value: Option<f64>,
    #[serde(rename = "simplePnl")]
    pub simple_pnl: Option<f64>,
    #[serde(rename = "simplePnlPcnt")]
    pub simple_pnl_pcnt: Option<f64>,
    #[serde(rename = "avgCostPrice")]
    pub avg_cost_price: Option<f64>,
    #[serde(rename = "avgEntryPrice")]
    pub avg_entry_price: Option<f64>,
    #[serde(rename = "breakEvenPrice")]
    pub break_even_price: Option<f64>,
    #[serde(rename = "marginCallPrice")]
    pub margin_call_price: Option<f64>,
    #[serde(rename = "liquidationPrice")]
    pub liquidation_price: Option<f64>,
    #[serde(rename = "bankruptPrice")]
    pub bankrupt_price: Option<f64>,
    pub timestamp: Option<DateTime<Utc>>,
    #[serde(rename = "lastPrice")]
    pub last_price: Option<f64>,
    #[serde(rename = "lastValue")]
    pub last_value: Option<i64>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
/// Best Bid/Offer Snapshots & Historical Bins
pub struct Quote {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    #[serde(rename = "bidSize")]
    pub bid_size: Option<i64>,
    #[serde(rename = "bidPrice")]
    pub bid_price: Option<f64>,
    #[serde(rename = "askPrice")]
    pub ask_price: Option<f64>,
    #[serde(rename = "askSize")]
    pub ask_size: Option<i64>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
/// Historical Settlement Data
pub struct Settlement {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    #[serde(rename = "settlementType")]
    pub settlement_type: Option<String>,
    #[serde(rename = "settledPrice")]
    pub settled_price: Option<f64>,
    #[serde(rename = "optionStrikePrice")]
    pub option_strike_price: Option<f64>,
    #[serde(rename = "optionUnderlyingPrice")]
    pub option_underlying_price: Option<f64>,
    pub bankrupt: Option<i64>,
    #[serde(rename = "taxBase")]
    pub tax_base: Option<i64>,
    #[serde(rename = "taxRate")]
    pub tax_rate: Option<f64>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
/// Exchange Statistics
pub struct Stats {
    #[serde(rename = "rootSymbol")]
    pub root_symbol: String,
    pub currency: Option<String>,
    pub volume24h: Option<i64>,
    pub turnover24h: Option<i64>,
    #[serde(rename = "openInterest")]
    pub open_interest: Option<i64>,
    #[serde(rename = "openValue")]
    pub open_value: Option<i64>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StatsHistory {
    pub date: DateTime<Utc>,
    #[serde(rename = "rootSymbol")]
    pub root_symbol: String,
    pub currency: Option<String>,
    pub volume: Option<i64>,
    pub turnover: Option<i64>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StatsUSD {
    #[serde(rename = "rootSymbol")]
    pub root_symbol: String,
    pub currency: Option<String>,
    pub turnover24h: Option<i64>,
    pub turnover30d: Option<i64>,
    pub turnover365d: Option<i64>,
    pub turnover: Option<i64>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
/// Individual & Bucketed Trades
pub struct Trade {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub side: Option<super::Side>,
    pub size: Option<i64>,
    pub price: Option<f64>,
    #[serde(rename = "tickDirection")]
    pub tick_direction: Option<String>,
    #[serde(rename = "trdMatchID")]
    pub trd_match_id: Option<Uuid>,
    #[serde(rename = "grossValue")]
    pub gross_value: Option<i64>,
    #[serde(rename = "homeNotional")]
    pub home_notional: Option<f64>,
    #[serde(rename = "foreignNotional")]
    pub foreign_notional: Option<f64>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
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
    #[serde(rename = "lastSize")]
    pub last_size: Option<i64>,
    pub turnover: Option<i64>,
    #[serde(rename = "homeNotional")]
    pub home_notional: Option<f64>,
    #[serde(rename = "foreignNotional")]
    pub foreign_notional: Option<f64>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub account: i64,
    pub currency: String,
    #[serde(rename = "prevDeposited")]
    pub prev_deposited: Option<i64>,
    #[serde(rename = "prevWithdrawn")]
    pub prev_withdrawn: Option<i64>,
    #[serde(rename = "prevTransferIn")]
    pub prev_transfer_in: Option<i64>,
    #[serde(rename = "prevTransferOut")]
    pub prev_transfer_out: Option<i64>,
    #[serde(rename = "prevAmount")]
    pub prev_amount: Option<i64>,
    #[serde(rename = "prevTimestamp")]
    pub prev_timestamp: Option<DateTime<Utc>>,
    #[serde(rename = "deltaDeposited")]
    pub delta_deposited: Option<i64>,
    #[serde(rename = "deltaWithdrawn")]
    pub delta_withdrawn: Option<i64>,
    #[serde(rename = "deltaTransferIn")]
    pub delta_transfer_in: Option<i64>,
    #[serde(rename = "deltaTransferOut")]
    pub delta_transfer_out: Option<i64>,
    #[serde(rename = "deltaAmount")]
    pub delta_amount: Option<i64>,
    pub deposited: Option<i64>,
    pub withdrawn: Option<i64>,
    #[serde(rename = "transferIn")]
    pub transfer_in: Option<i64>,
    #[serde(rename = "transferOut")]
    pub transfer_out: Option<i64>,
    pub amount: Option<i64>,
    #[serde(rename = "pendingCredit")]
    pub pending_credit: Option<i64>,
    #[serde(rename = "pendingDebit")]
    pub pending_debit: Option<i64>,
    #[serde(rename = "confirmedDebit")]
    pub confirmed_debit: Option<i64>,
    pub timestamp: Option<DateTime<Utc>>,
    pub addr: Option<String>,
    pub script: Option<String>,
    #[serde(rename = "withdrawalLock")]
    pub withdrawal_lock: Option<Vec<String>>
}
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Transaction {
    #[serde(rename = "transactID")]
    pub transact_id: Option<Uuid>,
    pub account: Option<i64>,
    pub currency: Option<String>,
    #[serde(rename = "transactType")]
    pub transact_type: Option<String>,
    pub amount: Option<i64>,
    pub fee: Option<i64>,
    #[serde(rename = "transactStatus")]
    pub transact_status: Option<String>,
    pub address: Option<String>,
    pub tx: Option<String>,
    pub text: Option<String>,
    #[serde(rename = "transactTime")]
    pub transact_time: Option<DateTime<Utc>>,
    pub timestamp: Option<DateTime<Utc>>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccessToken {
    pub id: String,
    /// time to live in seconds (2 weeks by default)
    pub ttl: Option<f64>,
    pub created: Option<DateTime<Utc>>,
    #[serde(rename = "userId")]
    pub user_id: Option<f64>
}
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Affiliate {
    pub account: Option<i64>,
    pub currency: Option<String>,
    #[serde(rename = "prevPayout")]
    pub prev_payout: Option<i64>,
    #[serde(rename = "prevTurnover")]
    pub prev_turnover: Option<i64>,
    #[serde(rename = "prevComm")]
    pub prev_comm: Option<i64>,
    #[serde(rename = "prevTimestamp")]
    pub prev_timestamp: Option<DateTime<Utc>>,
    #[serde(rename = "execTurnover")]
    pub exec_turnover: Option<i64>,
    #[serde(rename = "execComm")]
    pub exec_comm: Option<i64>,
    #[serde(rename = "totalReferrals")]
    pub total_referrals: Option<i64>,
    #[serde(rename = "totalTurnover")]
    pub total_turnover: Option<i64>,
    #[serde(rename = "totalComm")]
    pub total_comm: Option<i64>,
    #[serde(rename = "payoutPcnt")]
    pub payout_pcnt: Option<f64>,
    #[serde(rename = "pendingPayout")]
    pub pending_payout: Option<i64>,
    pub timestamp: Option<DateTime<Utc>>,
    #[serde(rename = "referrerAccount")]
    pub referrer_account: Option<f64>,
    #[serde(rename = "referralDiscount")]
    pub referral_discount: Option<f64>,
    #[serde(rename = "affiliatePayout")]
    pub affiliate_payout: Option<f64>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
/// Daily Quote Fill Ratio Statistic
pub struct QuoteFillRatio {
    pub date: DateTime<Utc>,
    pub account: Option<f64>,
    #[serde(rename = "quoteCount")]
    pub quote_count: Option<f64>,
    #[serde(rename = "dealtCount")]
    pub dealt_count: Option<f64>,
    #[serde(rename = "quotesMavg7")]
    pub quotes_mavg7: Option<f64>,
    #[serde(rename = "dealtMavg7")]
    pub dealt_mavg7: Option<f64>,
    #[serde(rename = "quoteFillRatioMavg7")]
    pub quote_fill_ratio_mavg7: Option<f64>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
/// Account Operations
pub struct User {
    pub id: Option<i32>,
    #[serde(rename = "ownerId")]
    pub owner_id: Option<i32>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub created: Option<DateTime<Utc>>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<DateTime<Utc>>,
    pub preferences: UserPreferences,
    #[serde(rename = "TFAEnabled")]
    pub t_f_a_enabled: Option<String>,
    #[serde(rename = "affiliateID")]
    pub affiliate_id: Option<String>,
    #[serde(rename = "pgpPubKey")]
    pub pgp_pub_key: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "geoipCountry")]
    pub geoip_country: Option<String>,
    #[serde(rename = "geoipRegion")]
    pub geoip_region: Option<String>,
    pub typ: Option<String>
}
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct UserCommissionsBySymbol(serde_json::Value);
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Margin {
    pub account: i64,
    pub currency: String,
    #[serde(rename = "riskLimit")]
    pub risk_limit: Option<i64>,
    #[serde(rename = "prevState")]
    pub prev_state: Option<String>,
    pub state: Option<String>,
    pub action: Option<String>,
    pub amount: Option<i64>,
    #[serde(rename = "pendingCredit")]
    pub pending_credit: Option<i64>,
    #[serde(rename = "pendingDebit")]
    pub pending_debit: Option<i64>,
    #[serde(rename = "confirmedDebit")]
    pub confirmed_debit: Option<i64>,
    #[serde(rename = "prevRealisedPnl")]
    pub prev_realised_pnl: Option<i64>,
    #[serde(rename = "prevUnrealisedPnl")]
    pub prev_unrealised_pnl: Option<i64>,
    #[serde(rename = "grossComm")]
    pub gross_comm: Option<i64>,
    #[serde(rename = "grossOpenCost")]
    pub gross_open_cost: Option<i64>,
    #[serde(rename = "grossOpenPremium")]
    pub gross_open_premium: Option<i64>,
    #[serde(rename = "grossExecCost")]
    pub gross_exec_cost: Option<i64>,
    #[serde(rename = "grossMarkValue")]
    pub gross_mark_value: Option<i64>,
    #[serde(rename = "riskValue")]
    pub risk_value: Option<i64>,
    #[serde(rename = "taxableMargin")]
    pub taxable_margin: Option<i64>,
    #[serde(rename = "initMargin")]
    pub init_margin: Option<i64>,
    #[serde(rename = "maintMargin")]
    pub maint_margin: Option<i64>,
    #[serde(rename = "sessionMargin")]
    pub session_margin: Option<i64>,
    #[serde(rename = "targetExcessMargin")]
    pub target_excess_margin: Option<i64>,
    #[serde(rename = "varMargin")]
    pub var_margin: Option<i64>,
    #[serde(rename = "realisedPnl")]
    pub realised_pnl: Option<i64>,
    #[serde(rename = "unrealisedPnl")]
    pub unrealised_pnl: Option<i64>,
    #[serde(rename = "indicativeTax")]
    pub indicative_tax: Option<i64>,
    #[serde(rename = "unrealisedProfit")]
    pub unrealised_profit: Option<i64>,
    #[serde(rename = "syntheticMargin")]
    pub synthetic_margin: Option<i64>,
    #[serde(rename = "walletBalance")]
    pub wallet_balance: Option<i64>,
    #[serde(rename = "marginBalance")]
    pub margin_balance: Option<i64>,
    #[serde(rename = "marginBalancePcnt")]
    pub margin_balance_pcnt: Option<f64>,
    #[serde(rename = "marginLeverage")]
    pub margin_leverage: Option<f64>,
    #[serde(rename = "marginUsedPcnt")]
    pub margin_used_pcnt: Option<f64>,
    #[serde(rename = "excessMargin")]
    pub excess_margin: Option<i64>,
    #[serde(rename = "excessMarginPcnt")]
    pub excess_margin_pcnt: Option<f64>,
    #[serde(rename = "availableMargin")]
    pub available_margin: Option<i64>,
    #[serde(rename = "withdrawableMargin")]
    pub withdrawable_margin: Option<i64>,
    pub timestamp: Option<DateTime<Utc>>,
    #[serde(rename = "grossLastValue")]
    pub gross_last_value: Option<i64>,
    pub commission: Option<f64>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
/// User communication SNS token
pub struct CommunicationToken {
    pub id: String,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "deviceToken")]
    pub device_token: String,
    pub channel: String
}
#[derive(Clone, Debug, Deserialize, Serialize)]
/// User Events for auditing
pub struct UserEvent {
    pub id: Option<f64>,
    #[serde(rename = "type")]
    pub r#type: String,
    pub status: String,
    #[serde(rename = "userId")]
    pub user_id: f64,
    #[serde(rename = "createdById")]
    pub created_by_id: Option<f64>,
    pub ip: Option<String>,
    #[serde(rename = "geoipCountry")]
    pub geoip_country: Option<String>,
    #[serde(rename = "geoipRegion")]
    pub geoip_region: Option<String>,
    #[serde(rename = "geoipSubRegion")]
    pub geoip_sub_region: Option<String>,
    #[serde(rename = "eventMeta")]
    pub event_meta: Option<EventMetaEventMeta>,
    pub created: DateTime<Utc>
}
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct EventMetaEventMeta(serde_json::Value);
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct UserPreferences {
    #[serde(rename = "alertOnLiquidations")]
    pub alert_on_liquidations: Option<bool>,
    #[serde(rename = "animationsEnabled")]
    pub animations_enabled: Option<bool>,
    #[serde(rename = "announcementsLastSeen")]
    pub announcements_last_seen: Option<DateTime<Utc>>,
    #[serde(rename = "chatChannelID")]
    pub chat_channel_id: Option<f64>,
    #[serde(rename = "colorTheme")]
    pub color_theme: Option<String>,
    pub currency: Option<String>,
    pub debug: Option<bool>,
    #[serde(rename = "disableEmails")]
    pub disable_emails: Option<Vec<String>>,
    #[serde(rename = "disablePush")]
    pub disable_push: Option<Vec<String>>,
    #[serde(rename = "hideConfirmDialogs")]
    pub hide_confirm_dialogs: Option<Vec<String>>,
    #[serde(rename = "hideConnectionModal")]
    pub hide_connection_modal: Option<bool>,
    #[serde(rename = "hideFromLeaderboard")]
    pub hide_from_leaderboard: Option<bool>,
    #[serde(rename = "hideNameFromLeaderboard")]
    pub hide_name_from_leaderboard: Option<bool>,
    #[serde(rename = "hideNotifications")]
    pub hide_notifications: Option<Vec<String>>,
    pub locale: Option<String>,
    #[serde(rename = "msgsSeen")]
    pub msgs_seen: Option<Vec<String>>,
    #[serde(rename = "orderBookBinning")]
    pub order_book_binning: Option<OrderBookBinningOrderBookBinning>,
    #[serde(rename = "orderBookType")]
    pub order_book_type: Option<String>,
    #[serde(rename = "orderClearImmediate")]
    pub order_clear_immediate: Option<bool>,
    #[serde(rename = "orderControlsPlusMinus")]
    pub order_controls_plus_minus: Option<bool>,
    #[serde(rename = "showLocaleNumbers")]
    pub show_locale_numbers: Option<bool>,
    pub sounds: Option<Vec<String>>,
    #[serde(rename = "strictIPCheck")]
    pub strict_i_p_check: Option<bool>,
    #[serde(rename = "strictTimeout")]
    pub strict_timeout: Option<bool>,
    #[serde(rename = "tickerGroup")]
    pub ticker_group: Option<String>,
    #[serde(rename = "tickerPinned")]
    pub ticker_pinned: Option<bool>,
    #[serde(rename = "tradeLayout")]
    pub trade_layout: Option<String>
}
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct OrderBookBinningOrderBookBinning(serde_json::Value);