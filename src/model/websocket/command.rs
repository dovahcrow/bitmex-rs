use hyper::Method;
use url::Url;
use serde_derive::{Deserialize, Serialize};

use super::Topic;
use crate::consts::WS_URL;
use crate::error::Result;
use crate::BitMEX;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", content = "args")]
#[serde(rename_all = "camelCase")]
pub enum Command {
    Subscribe(Vec<Topic>),
    Unsubscribe(Vec<Topic>),
    #[serde(rename = "authKeyExpires")]
    Authenticate(String, i64, String), // ApiKey, Expires, Signature
    CancelAllAfter(i64),
    Ping,
}

impl Command {
    pub fn authenticate(bm: &BitMEX, expires: i64) -> Result<Command> {
        let (key, sig) = bm.transport.signature(&Method::GET, expires, &Url::parse(&WS_URL)?, "")?;
        Ok(Command::Authenticate(key.to_string(), expires, sig))
    }
}
