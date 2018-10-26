use std::env::var;

use hyper::Method;
use log::warn;
use url::Url;

use super::Topic;
use crate::consts::{WS_URL, WS_URL_TESTNET};
use crate::error::Result;
use crate::BitMEX;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", content = "args")]
#[serde(rename_all = "lowercase")]
pub enum Command {
    Subscribe(Vec<Topic>),
    Unsubscribe(Vec<Topic>),
    #[serde(rename = "authKeyExpires")]
    Authenticate(String, i64, String), // ApiKey, Expires, Signature
    Ping,
}

impl Command {
    pub fn authenticate(bm: &BitMEX, expires: i64) -> Result<Command> {
        let base = if var("BITMEX_TESTNET").unwrap_or("0".to_string()) == "0" {
            WS_URL
        } else {
            warn!("Your are using BitMEX test net");
            WS_URL_TESTNET
        };
        let (key, sig) = bm.transport.signature(Method::GET, expires, &Url::parse(base)?, "")?;
        Ok(Command::Authenticate(key.to_string(), expires, sig))
    }
}
