// pub mod announcement;
// pub mod api_key;
// pub mod chat;
pub mod definitions;
// pub mod execution;
// pub mod funding;
// pub mod global_notification;
// pub mod instrument;
// pub mod insurance;
// pub mod leaderboard;
// pub mod liquidation;
// pub mod order;
// pub mod order_book;
// pub mod position;
mod public;
// pub mod quote;
// pub mod settlement;
pub mod requests;
pub mod swagger;
// pub mod trade;
// pub mod user;
// pub mod user_event;
// pub mod websocket;
pub use self::definitions::*;
pub use self::public::{
    BinSize, ContingencyType, ExecInst, OrdType, PegPriceType, Side, TimeInForce, Vararg,
};
pub use self::requests::*;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;

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
