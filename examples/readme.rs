use chrono::{Duration, Utc};
use failure::Fallible;
use futures::sink::SinkExt;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    ::env_logger::init();

    // This will give you a BitMEX instance, which the only purpose is to create connection.
    let bm = bitmex::rest::BitMEXRest::with_credential(
        &std::env::var("BITMEX_KEY")?,
        &std::env::var("BITMEX_SECRET")?,
    );

    // All the requests to BitMEX server afterwards will go through HTTP Restful API.

    // The request models reside in "bitmex::models" module, with the
    // naming convention of "Method+camelCase(endpoint)+Request", e.g. "GET /trade/bucketed" would be
    // "bitmex::models::GetTradeBucketedRequest" in bitmex-rs.
    let req = bitmex::rest::GetTradeBucketedRequest {
        bin_size: Some(bitmex::rest::BinSize::D1),
        ..Default::default()
    };

    // Request to BitMEX server is made by giving "BitMEX::request" the request object.
    // The return type of "BitMEX::request" is a future of the response so that you can await on it.
    let resp = bm.request(req).await?;
    println!("Bucketed trades: {:?}", resp);

    // A websocket is created by "BitMEX::websocket".
    let mut ws = bitmex::websocket::BitMEXWebsocket::with_credential(
        &std::env::var("BITMEX_KEY")?,
        &std::env::var("BITMEX_SECRET")?,
    )
    .await?;

    // The websocket is a duplex channel which means you can send "bitmex::websocket::Command" to BitMEX and
    // receive "bitmex::websocket::Message" from BitMEX using it.
    let expires = (Utc::now() + Duration::seconds(30)).timestamp();
    ws.send(bitmex::websocket::Command::authenticate(expires as u64))
        .await?;

    // In order to get the ws messages, just poll the ws stream.
    while let Some(message) = ws.next().await {
        println!("Subscription message received {:?}", message);
    }

    Ok(())
}
