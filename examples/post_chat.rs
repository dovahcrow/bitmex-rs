extern crate bitmex;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate tokio;

use std::env::var;

use bitmex::model::chat::PostChatRequest;
use bitmex::{BitMEX, Result};
use tokio::runtime::current_thread::Runtime;

fn main() -> Result<()> {
    ::dotenv::dotenv().ok();
    ::env_logger::init();

    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.post_chat(PostChatRequest {
        message: "hello2 from bot".to_string(),
        ..Default::default()
    })?;

    let ret = rt.block_on(fut)?;
    println!("{:?}", ret);
    Ok(())
}
