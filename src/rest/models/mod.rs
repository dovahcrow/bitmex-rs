mod definitions;
mod requests;
pub mod swagger;
pub use self::definitions::*;
pub use self::requests::*;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub trait Request: Serialize {
    const METHOD: Method;
    const SIGNED: bool = false;
    const ENDPOINT: &'static str;
    const HAS_PAYLOAD: bool = true;
    type Response: DeserializeOwned;

    #[inline]
    fn no_payload(&self) -> bool {
        !Self::HAS_PAYLOAD
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Side {
    Buy,
    Sell,
    #[serde(rename = "")]
    Unknown, // BitMEX sometimes has empty side due to unknown reason
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BinSize {
    #[serde(rename = "1m")]
    M1,
    #[serde(rename = "5m")]
    M5,
    #[serde(rename = "1h")]
    H1,
    #[serde(rename = "1d")]
    D1,
}

impl Default for BinSize {
    fn default() -> Self {
        self::BinSize::D1
    }
}

/// http://fixwiki.org/fixwiki/PegPriceType
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum PegPriceType {
    LastPeg,
    OpeningPeg,
    MidPricePeg,
    MarketPeg,
    PrimaryPeg,
    PegToVWAP,
    TrailingStopPeg,
    PegToLimitPrice,
    ShortSaleMinPricePeg,
    #[serde(rename = "")]
    Unknown, // BitMEX sometimes has empty due to unknown reason
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

/// https://www.onixs.biz/fix-dictionary/5.0.SP2/tagNum_59.html
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum TimeInForce {
    Day,
    GoodTillCancel,
    AtTheOpening,
    ImmediateOrCancel,
    FillOrKill,
    GoodTillCrossing,
    GoodTillDate,
    AtTheClose,
    GoodThroughCrossing,
    AtCrossing,
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
    #[serde(rename = "")]
    Unknown, // BitMEX sometimes has empty due to unknown reason
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ContingencyType {
    OneCancelsTheOther,
    OneTriggersTheOther,
    OneUpdatesTheOtherAbsolute,
    OneUpdatesTheOtherProportional,
    #[serde(rename = "")]
    Unknown, // BitMEX sometimes has empty due to unknown reason
}
