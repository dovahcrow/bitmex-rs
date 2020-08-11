mod consts;
mod credential;
mod error;
pub mod rest;
pub mod websocket;

pub use crate::error::BitMEXError;

use fehler::throws;

pub const API_VERSION: &str = "1.2.0";
pub const SWAGGER_URL: &str = "https://www.bitmex.com/api/explorer/swagger.json";

#[throws(failure::Error)]
pub async fn check_version() -> bool {
    let desc = crate::rest::BitMEXRest::new().get_swagger().await?;
    desc.info.version == API_VERSION
}
