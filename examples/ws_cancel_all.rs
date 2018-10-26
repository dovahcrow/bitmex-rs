extern crate bitmex;
extern crate chrono;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate tokio;
extern crate tungstenite;

use bitmex::model::websocket::Command;
use bitmex::{BitMEX, Result};
use chrono::{Duration, Utc};
use futures::{Future, Sink, Stream};
use std::env::var;
use tokio::runtime::current_thread::Runtime;

fn main() -> Result<()> {
    ::dotenv::dotenv().ok();
    ::env_logger::init();

    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let job = bm
        .websocket()
        .and_then(|ws| {
            println!("WebSocket handshake has been successfully completed");
            let expires = (Utc::now() + Duration::seconds(30)).timestamp();
            ws.send(Command::authenticate(&bm, expires).unwrap())
        }).and_then(|ws| ws.send(Command::CancelAllAfter(365 * 24 * 60 * 60 * 1000)))
        .and_then(|ws| ws.map(|msg| println!("{:?}", msg)).collect())
        .map_err(|e| {
            println!("Error during the websocket handshake occurred: {}", e);
            e
        });

    rt.block_on(job)?;
    Ok(())
}
