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
    //kSd8p10yUJJMavFZ3V-vqNCJ
    //t2vD6HhOeTK8YjgP4AQ-QX69mNVLTAXy6tm4pT_c9zixbV0-
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let job = bm
        .websocket()
        .and_then(|ws| {
            println!("WebSocket handshake has been successfully completed");
            ws.send(Command::Subscribe(vec![
                Topic::Chat,
                Topic::OrderBookL2(Some("XBTUSD".to_string())),
                Topic::Connected,
                Topic::Liquidation,
                Topic::QuoteBin1m,
                Topic::TradeBin1m,
                Topic::Trade(None),
                Topic::Settlement,
                Topic::OrderBook10,
                Topic::Announcement,
            ]))
        })
        .and_then(|ws| ws.map(|msg| println!("{:?}", msg)).collect())
        .map_err(|e| {
            println!("Error during the websocket handshake occurred: {}", e);
            e
        });

    rt.block_on(job)?;
    Ok(())
}
