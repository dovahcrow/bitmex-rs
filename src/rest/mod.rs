mod client;
mod models;

pub use client::{BitMEXRest, BitMEXRestBuilder};
pub use models::*;

use crate::error::BitMEXError;
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
