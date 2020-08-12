use failure::Fail;
use serde::{Deserialize, Serialize};

#[derive(Debug, Fail, Serialize, Deserialize, Clone)]
pub enum BitMEXError {
    #[fail(display = "No Api key set for private api")]
    NoApiKeySet,
    #[fail(display = "{} error message from BitMEX server: {}", name, message)]
    RemoteError { message: String, name: String },
    #[fail(display = "Websocket closed")]
    WebsocketClosed,
    #[fail(display = "Unexpected websocket binary content {:?}", _0)]
    UnexpectedWebsocketBinaryContent(Vec<u8>),
    #[fail(display = "Cannot parse topic {:?}", _0)]
    ParseTopicError(String),
    #[fail(display = "Error from websocket. {}: {}", status, error)]
    WebsocketError { status: i64, error: String },
}
