use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use super::Command;

// Text("{\"success\":true,\"subscribe\":\"chat\",\"request\":{\"args\":[\"chat\"],\"op\":\"subscribe\"}}")
// Text("{\"table\":\"chat\",\"action\":\"insert\",\"keys\":[\"id\"],\"data\":[{\"channelID\":4,\"date\":\"2018-10-26T05:09:44.159Z\",\"fromBot\":false,\"html\":\"ㅋㅋㅋㅋㅋ ETF 드립 ㅈㄴ웃기네\\n\",\"id\":21699228,\"message\":\"ㅋㅋㅋㅋㅋ ETF 드립 ㅈㄴ웃기네\",\"user\":\"xixixiaqs\"}],\"filterKey\":\"channelID\"}")
// Text("{\"info\":\"Welcome to the BitMEX Realtime API.\",\"version\":\"2018-10-23T18:33:47.000Z\",\"timestamp\":\"2018-10-26T05:09:14.006Z\",\"docs\":\"https://www.bitmex.com/app/wsAPI\",\"limit\":{\"remaining\":38}}")
// {"success":true,"unsubscribe":"chat","request":{"op":"unsubscribe","args":["chat"]}}
// {"status":400,"error":"Failed to decode incoming data: Unexpected token a in JSON at position 0. Please see the documentation at https://www.bitmex.com/app/wsAPI.","meta":{}}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Message {
    Success(SuccessMessage),
    Error(ErrorMessage),
    Table(Box<TableMessage<Value>>),
    Info(InfoMessage),
    CancelAllAfter(CancelAllAfterMessage),
    Pong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessMessage {
    pub success: bool,
    pub subscribe: Option<String>,
    pub request: Command,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllAfterMessage {
    pub now: DateTime<Utc>,
    pub cancel_time: DateTime<Utc>,
    pub request: Command,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoMessage {
    pub info: String,
    pub version: DateTime<Utc>,
    pub timestamp: DateTime<Utc>,
    pub docs: String,
    pub limit: Limit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Limit {
    pub remaining: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub status: i64,
    pub error: String,
    pub request: Option<Command>,
    pub meta: Value,
}

//Text("{\"table\":\"chat\",\"action\":\"insert\",\"keys\":[\"id\"],\"data\":[{\"channelID\":4,\"date\":\"2018-10-26T05:09:44.159Z\",\"fromBot\":false,\"html\":\"ㅋㅋㅋㅋㅋ ETF 드립 ㅈㄴ웃기네\\n\",\"id\":21699228,\"message\":\"ㅋㅋㅋㅋㅋ ETF 드립 ㅈㄴ웃기네\",\"user\":\"xixixiaqs\"}],\"filterKey\":\"channelID\"}")

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableMessage<T> {
    pub table: String,
    pub action: Action,
    pub data: Vec<T>,
    pub keys: Option<Vec<String>>,
    pub foreign_keys: Option<HashMap<String, String>>,
    pub types: Option<HashMap<String, String>>,
    pub filter: Option<TableFilter>,
    pub attributes: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableFilter {
    pub account: Option<i64>,
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    Insert,
    Partial,
    Update,
    Delete,
}
