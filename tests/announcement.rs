use bitmex::models::GetAnnouncementRequest;
use bitmex::BitMEX;
use failure::Fallible;
use tokio::runtime::Runtime;

#[test]
fn get_announcement() -> Fallible<()> {
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.request(GetAnnouncementRequest { columns: None })?;

    let _ = rt.block_on(fut)?;
    Ok(())
}

#[test]
fn get_announcement_urgent() -> Fallible<()> {
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_announcement_urgent()?;

    let _ = rt.block_on(fut)?;
    Ok(())
}
