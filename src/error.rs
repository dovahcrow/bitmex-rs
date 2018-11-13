use std::result::Result as StdResult;

use failure::Error;
use serde::de::DeserializeOwned;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum BitMEXResponse<T> {
    Success(T),
    Error { error: BitMEXResponseError },
}

impl<T: DeserializeOwned> BitMEXResponse<T> {
    pub fn into_result(self) -> StdResult<T, BitMEXResponseError> {
        match self {
            BitMEXResponse::Success(t) => StdResult::Ok(t),
            BitMEXResponse::Error { error: e } => StdResult::Err(e),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Fail)]
#[fail(display = "BitMEX error: {}", message)]
pub struct BitMEXResponseError {
    pub message: String,
    pub name: String,
}

#[derive(Debug, Fail, Serialize, Deserialize, Clone)]
pub enum BitMEXError {
    #[fail(display = "No Api key set for private api")]
    NoApiKeySet,
}
