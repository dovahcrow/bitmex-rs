use bitmex::models::{GetChatRequest, PostChatRequest};
use bitmex::BitMEX;
use failure::Fallible;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_chat() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_chat(GetChatRequest {
        count: 1,
        channel_id: Some(1),
        ..Default::default()
    })?;

    let _ = rt.block_on(fut)?;
    Ok(())
}

#[test]
#[ignore] // My test account was banned from chatting on testnet
fn post_chat() -> Fallible<()> {
    ::dotenv::dotenv().ok();

    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.post_chat(PostChatRequest {
        message: "\n---- IGNORE ME ----\nbitmex-rs library testing\n---- IGNORE ME ----".into(),
        channel_id: 1,
        ..Default::default()
    })?;

    let _ = rt.block_on(fut)?;
    Ok(())
}

#[test]
fn get_chat_channels() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_chat_channels()?;

    let _ = rt.block_on(fut)?;
    Ok(())
}

#[test]
fn get_chat_connected() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_chat_connected()?;

    let _ = rt.block_on(fut)?;
    Ok(())
}
