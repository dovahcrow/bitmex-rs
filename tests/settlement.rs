extern crate bitmex;
extern crate dotenv;
extern crate tokio;

use std::env::var;

use bitmex::model::settlement::GetSettlementRequest;
use bitmex::{BitMEX, Result};
use tokio::runtime::Runtime;

#[test]
fn get_settlement() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    assert!(rt.block_on(bm.get_settlement(GetSettlementRequest { ..Default::default() })?).is_err());
    Ok(())
}
