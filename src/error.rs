use failure::Fail;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BitMEXErrorResponse {
    pub error: BitMEXErrorMessage,
}

#[derive(Deserialize, Serialize, Debug, Clone, Fail)]
#[fail(display = "BitMEX error: {}", message)]
pub struct BitMEXErrorMessage {
    pub message: String,
    pub name: String,
}

#[derive(Debug, Fail, Serialize, Deserialize, Clone)]
pub enum BitMEXError {
    #[fail(display = "No Api key set for private api")]
    NoApiKeySet,
}
