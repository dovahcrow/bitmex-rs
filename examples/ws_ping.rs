use bitmex::websocket::{BitMEXWebsocket, Command};
use failure::Fallible;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use std::env::var;

#[tokio::main]
async fn main() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    ::env_logger::init();

    let mut client =
        BitMEXWebsocket::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?).await?;

    println!("WebSocket handshake has been successfully completed");

    client.send(Command::Ping).await?;

    while let Some(msg) = client.next().await {
        println!("{:?}", msg);
    }
    Ok(())
}
