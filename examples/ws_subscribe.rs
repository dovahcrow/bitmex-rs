use bitmex::websocket::{Command, Topic};
use bitmex::BitMEX;
use failure::Fallible;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use std::env::var;

#[tokio::main]
async fn main() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    ::env_logger::init();

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let mut client = bm.websocket().await?;
    println!("WebSocket handshake has been successfully completed");

    client
        .send(Command::Subscribe(vec![
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
        .await?;

    while let Some(msg) = client.next().await {
        println!("{:?}", msg);
    }
    Ok(())
}
