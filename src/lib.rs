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

pub use client::BitMEX;
pub use error::Result;
