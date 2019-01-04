extern crate bitmex;
extern crate dotenv;
extern crate tokio;

use std::env::var;

use bitmex::model::user_event::GetUserEventRequest;
use bitmex::{BitMEX, Result};
use tokio::runtime::Runtime;

#[test]
fn get_user_event() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.get_user_event(GetUserEventRequest::default())?)?;
    Ok(())
}
