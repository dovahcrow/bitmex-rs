mod client;
mod consts;
mod error;
pub mod models;
mod transport;
pub mod websocket;

pub use crate::client::BitMEX;
pub use crate::websocket::BitMEXWebsocket;
use failure::Fallible;

pub const API_VERSION: &str = "1.2.0";
pub const SWAGGER_URL: &str = "https://www.bitmex.com/api/explorer/swagger.json";

pub async fn check_version() -> Fallible<bool> {
    let desc = transport::Transport::new().get_swagger().await?;
    Ok(desc.info.version == API_VERSION)
}
