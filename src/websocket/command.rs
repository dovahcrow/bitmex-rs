use super::Topic;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", content = "args")]
#[serde(rename_all = "camelCase")]
pub enum Command {
    Subscribe(Vec<Topic>),
    Unsubscribe(Vec<Topic>),
    #[serde(rename = "authKeyExpires")]
    Authenticate(Option<String>, u64, Option<String>),
    CancelAllAfter(u64),
    Ping,
}

impl Command {
    pub fn authenticate(expires: u64) -> Command {
        Command::Authenticate(None, expires, None)
    }
    pub fn subscribe(topics: Vec<Topic>) -> Command {
        Command::Subscribe(topics)
    }
    pub fn unsubscribe(topics: Vec<Topic>) -> Command {
        Command::Unsubscribe(topics)
    }
    pub fn cancel_all_after(millisecs: u64) -> Command {
        Command::CancelAllAfter(millisecs)
    }
    pub fn ping() -> Command {
        Command::Ping
    }
}
