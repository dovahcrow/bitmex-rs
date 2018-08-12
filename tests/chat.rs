extern crate bitmex;
extern crate tokio;

use bitmex::{BitMEX, Result};
use tokio::runtime::Runtime;

#[test]
fn test_get_chat() -> Result<()> {
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_chat(1, None, 1)?;

    let _ = rt.block_on(fut)?;
    Ok(())
}
