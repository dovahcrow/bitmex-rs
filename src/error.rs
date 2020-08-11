use failure::Fail;
use serde::{Deserialize, Serialize};
use std::convert::From;

// The error response from bitmex;
#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct BitMEXErrorResponse {
    pub(crate) error: BitMEXErrorMessage,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct BitMEXErrorMessage {
    pub(crate) message: String,
    pub(crate) name: String,
}

impl From<BitMEXErrorMessage> for BitMEXError {
    fn from(msg: BitMEXErrorMessage) -> BitMEXError {
        BitMEXError::RemoteError {
            message: msg.message,
            name: msg.name,
        }
    }
}

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
}
