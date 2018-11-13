extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_json;
extern crate tokio;
extern crate url;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
extern crate chrono;
extern crate hex;
#[macro_use]
extern crate lazy_static;
extern crate log;
extern crate ring;
extern crate tokio_tungstenite;
extern crate tungstenite;
extern crate uuid;

mod client;
mod consts;
mod error;
pub mod model;
mod transport;

pub use client::websocket::BitMEXWebsocket;
pub use client::BitMEX;

pub use error::Result;
use failure::Error;
use futures::{Future, IntoFuture};

pub const API_VERSION: &'static str = "1.2.0";
pub const SWAGGER_URL: &'static str = "https://www.bitmex.com/api/explorer/swagger.json";

pub fn check_version() -> impl Future<Item = bool, Error = Error> {
    let resp = transport::Transport::new().get_swagger().into_future().flatten();
    resp.map(|desc| desc.info.version == API_VERSION)
}
