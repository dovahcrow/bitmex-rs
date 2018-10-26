extern crate bitmex;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate tokio;
extern crate tungstenite;

use bitmex::model::websocket::{Command, Topic};
use bitmex::{BitMEX, Result};
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
        .and_then(move |mut ws| {
            println!("WebSocket handshake has been successfully completed");
            ws.start_send(Command::Subscribe(vec![
                Topic::Chat,
                Topic::OrderBookL2(Some("XBTUSD".to_string())),
                Topic::Connected,
                Topic::Liquidation,
                Topic::QuoteBin1m,
                Topic::TradeBin1m,
                Topic::Trade,
                Topic::Settlement,
                Topic::OrderBook10,
                Topic::Announcement,
            ])).unwrap();
            ws.map(|msg| println!("{:?}", msg)).collect().from_err()
        }).map_err(|e| {
            println!("Error during the websocket handshake occurred: {}", e);
            e
        });

    rt.block_on(job)?;
    Ok(())
}
