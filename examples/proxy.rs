use bitmex::rest::BitMEXRest;
use bitmex::rest::GetFundingRequest;
use failure::Fallible;
use reqwest::{Client, Proxy};

#[tokio::main]
async fn main() -> Fallible<()> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let proxy = Proxy::http("https://secure.example")?;
    let transport = Client::builder().proxy(proxy).build()?;

    let bm = BitMEXRest::builder().client(transport).build().unwrap();

    let res = bm
        .request(GetFundingRequest {
            symbol: Some("XBT".to_string()),
            ..Default::default()
        })
        .await?;

    println!("{:?}", res);
    Ok(())
}
