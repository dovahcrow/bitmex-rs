pub mod core;
mod client; //此处会扫描src/client.rs 或者 /src/client/mod.rs
mod consts;
mod error;
pub mod model;
mod transport;


use failure::Error;
use futures::{Future, IntoFuture};

pub use crate::client::websocket::BitMEXWebsocket;
pub use crate::client::BitMEX;
pub use crate::error::Result;

pub const API_VERSION: &str = "1.2.0";
pub const SWAGGER_URL: &str = "https://www.bitmex.com/api/explorer/swagger.json";

pub fn check_version() -> impl Future<Item = bool, Error = Error> {
    let resp = transport::Transport::new().get_swagger().into_future().flatten();
    resp.map(|desc| desc.info.version == API_VERSION)
}

pub fn print_version() {
    println!("the vesion is 0.1");
}
