use bitmex::BitMEX;
use failure::Fallible;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_global_notification() -> Fallible<()> {
    // This will fail for error access denied
    ::dotenv::dotenv().ok();

    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.get_global_notification()?;

    assert!(rt.block_on(fut).is_err());
    Ok(())
}
