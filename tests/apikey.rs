extern crate bitmex;
extern crate dotenv;
extern crate env_logger;
extern crate tokio;

use std::env::var;

use bitmex::{BitMEX, Result};
use tokio::runtime::Runtime;

#[test]
fn test_list_api_key() -> Result<()> {
    ::dotenv::dotenv().ok();

    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.get_api_key()?;

    let _ = rt.block_on(fut)?;
    Ok(())
}
