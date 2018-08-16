use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Announcement {
    id: usize,
    link: String,
    title: String,
    content: String,
    date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    id: String,
    secret: Option<String>, // The document claims this field's existence, but actually not
    name: String,
    nonce: usize,
    cidr: String,
    permissions: Vec<String>,
    enabled: bool,
    user_id: usize,
    created: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    id: usize,
    date: DateTime<Utc>,
    user: String,
    message: String,
    html: String,
    from_bot: bool,
    #[serde(rename = "channelID")]
    channel_id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Channel {
    id: usize,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectedUsers {
    users: usize,
    bots: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Success {
    success: bool,
}

#[derive(Copy, Clone, Debug)]
pub enum ApiKeyPermission {
    Order,
    OrderCancel,
    Withdraw,
}

impl ToString for ApiKeyPermission {
    fn to_string(&self) -> String {
        match self {
            ApiKeyPermission::Order => "order".to_string(),
            ApiKeyPermission::OrderCancel => "orderCancel".to_string(),
            ApiKeyPermission::Withdraw => "withdraw".to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Side {
    Buy,
    Sell,
}

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

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderRequest {
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderResponse {
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
    pub simple_leaves_qty: f64,
    pub leaves_qty: f64,
    pub simple_cum_qty: f64,
    pub cum_qty: f64,
    pub avg_px: Option<f64>,
    pub multi_leg_reporting_type: String,
    pub text: String,
    pub transact_time: DateTime<Utc>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderRequest {
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
pub struct AmendOrderResponse {
    #[serde(rename = "orderID")]
    pub order_id: Uuid,
    #[serde(rename = "clOrdID")]
    pub cl_ord_id: String,
    #[serde(rename = "clOrdLinkID")]
    pub cl_ord_link_id: String,
    pub account: f64,
    pub symbol: String,
    pub side: String,
    pub simple_order_qty: f64,
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
