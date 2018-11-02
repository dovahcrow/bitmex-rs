extern crate bitmex;
extern crate tokio;

use bitmex::model::announcement::GetAnnouncementRequest;
use bitmex::{BitMEX, Result};
use tokio::runtime::Runtime;

#[test]
fn get_announcement() -> Result<()> {
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_announcement(GetAnnouncementRequest { ..Default::default() })?;

    let _ = rt.block_on(fut)?;
    Ok(())
}

#[test]
fn get_announcement_urgent() -> Result<()> {
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_announcement_urgent()?;

    let _ = rt.block_on(fut)?;
    Ok(())
}
