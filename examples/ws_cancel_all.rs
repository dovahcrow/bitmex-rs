use bitmex::websocket::{BitMEXWebsocket, Command};
use chrono::{Duration, Utc};
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
    let expires = (Utc::now() + Duration::seconds(30)).timestamp();
    client.send(Command::authenticate(expires as u64)).await?;

    client
        .send(Command::CancelAllAfter(365 * 24 * 60 * 60 * 1000))
        .await?;

    while let Some(msg) = client.next().await {
        println!("{:?}", msg);
    }
    Ok(())
}
