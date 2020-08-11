use failure::Fallible;
use std::env::var;

use bitmex::rest::BitMEXRest;
use bitmex::rest::PostChatRequest;

#[tokio::main]
async fn main() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    ::env_logger::init();

    let bm = BitMEXRest::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let ret = bm
        .request(PostChatRequest {
            message: "hello2 from bot".to_string(),
            channel_id: Some(1f64),
        })
        .await?;

    println!("{:?}", ret);
    Ok(())
}
