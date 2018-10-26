use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;

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
    Table(TableMessage<Value>),
    Info(InfoMessage),
    CancelAllAfter(CancelAllAfterMessage),
    Pong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessMessage {
    success: bool,
    subscribe: Option<String>,
    request: Command,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllAfterMessage {
    now: DateTime<Utc>,
    cancel_time: DateTime<Utc>,
    request: Command,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoMessage {
    info: String,
    version: DateTime<Utc>,
    timestamp: DateTime<Utc>,
    docs: String,
    limit: Limit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Limit {
    remaining: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMessage {
    status: i64,
    error: String,
    request: Option<Command>,
    meta: Value,
}

//Text("{\"table\":\"chat\",\"action\":\"insert\",\"keys\":[\"id\"],\"data\":[{\"channelID\":4,\"date\":\"2018-10-26T05:09:44.159Z\",\"fromBot\":false,\"html\":\"ㅋㅋㅋㅋㅋ ETF 드립 ㅈㄴ웃기네\\n\",\"id\":21699228,\"message\":\"ㅋㅋㅋㅋㅋ ETF 드립 ㅈㄴ웃기네\",\"user\":\"xixixiaqs\"}],\"filterKey\":\"channelID\"}")

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableMessage<T> {
    table: String,
    action: Action,
    data: Vec<T>,
    keys: Option<Vec<String>>,
    foreign_keys: Option<HashMap<String, String>>,
    types: Option<HashMap<String, String>>,
    filter: Option<TableFilter>,
    attributes: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableFilter {
    account: Option<i64>,
    symbol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    Insert,
    Partial,
    Update,
    Delete,
}
