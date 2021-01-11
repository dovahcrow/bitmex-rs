use bitmex::rest::BitMEXRest;
use bitmex::rest::GetUserEventRequest;
use failure::Fallible;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_user_event() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let rt = Runtime::new()?;
    let bm = BitMEXRest::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.request(GetUserEventRequest::default()))?;
    Ok(())
}
