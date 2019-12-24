use bitmex::models::{Command, Topic};
use bitmex::BitMEX;
use futures::{Future, Sink, Stream};
use std::env::var;
use tokio::runtime::current_thread::Runtime;

fn main() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    ::env_logger::init();

    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let job = bm
        .websocket()
        .and_then(|ws| {
            println!("WebSocket handshake has been successfully completed");
            ws.send(Command::Subscribe(vec![
                // Topic::OrderBookL2_25(Some("XBTUSD".to_string())),
                Topic::Trade(Some("XBTUSD".to_string())),
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
