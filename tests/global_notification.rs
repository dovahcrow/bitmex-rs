use bitmex::models::GetGlobalNotificationRequest;
use bitmex::BitMEX;
use failure::Fallible;
use log::debug;
use std::env::var;
use tokio::runtime::Runtime;

// This will fail for error access denied
#[test]
fn get_global_notification() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();

    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.request(GetGlobalNotificationRequest {});
    let ret = rt.block_on(fut);

    debug!("{:?}", ret);
    assert!(ret.is_err());
    Ok(())
}
