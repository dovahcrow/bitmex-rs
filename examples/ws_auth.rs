extern crate bitmex;
extern crate chrono;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate tokio;
extern crate tungstenite;

use std::env::var;

use bitmex::model::websocket::{Command, Topic};
use bitmex::{BitMEX, Result};
use chrono::{Duration, Utc};
use futures::{Future, Sink, Stream};
use tokio::runtime::current_thread::Runtime;

fn main() -> Result<()> {
    ::dotenv::dotenv().ok();
    ::env_logger::init();

    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let job = bm
        .websocket()
        .and_then(move |mut ws| {
            println!("WebSocket handshake has been successfully completed");
            let expires = (Utc::now() + Duration::seconds(30)).timestamp();
            ws.start_send(Command::authenticate(&bm, expires).unwrap()).unwrap();
            ws.start_send(Command::Subscribe(vec![Topic::Position])).unwrap();
            ws.map(|msg| println!("{:?}", msg)).collect().from_err()
        }).map_err(|e| {
            println!("Error during the websocket handshake occurred: {}", e);
            e
        });

    rt.block_on(job)?;
    Ok(())
}
