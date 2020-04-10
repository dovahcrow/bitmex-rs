mod client;
mod consts;
mod error;
pub mod models;
pub mod websocket;

pub use crate::client::{BitMEX, BitMEXBuilder};
pub use crate::error::BitMEXError;
pub use crate::websocket::BitMEXWebsocket;

use fehler::throws;

pub const API_VERSION: &str = "1.2.0";
pub const SWAGGER_URL: &str = "https://www.bitmex.com/api/explorer/swagger.json";

#[throws(failure::Error)]
pub async fn check_version() -> bool {
    let desc = client::BitMEX::new().get_swagger().await?;
    desc.info.version == API_VERSION
}
