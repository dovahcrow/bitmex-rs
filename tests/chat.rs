extern crate bitmex;
extern crate tokio;

use bitmex::model::chat::GetChatRequest;
use bitmex::{BitMEX, Result};
use tokio::runtime::Runtime;

#[test]
fn test_get_chat() -> Result<()> {
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
