extern crate bitmex;
extern crate tokio;

use bitmex::model::funding::GetFundingRequest;
use bitmex::{BitMEX, Result};
use tokio::runtime::Runtime;

#[test]
fn get_funding() -> Result<()> {
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_funding(GetFundingRequest {
        symbol: Some("XBT".to_string()),
        ..Default::default()
    })?;

    let res = rt.block_on(fut)?;
    println!("{:?}", res);
    Ok(())
}
