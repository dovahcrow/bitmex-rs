extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate tokio;
extern crate url;
// #[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate chrono;
// #[macro_use]
// extern crate derive_more;
extern crate hex;
extern crate ring;

mod client;
mod error;
pub mod model;
mod transport;

pub use client::BitMEX;
pub use error::Result;
