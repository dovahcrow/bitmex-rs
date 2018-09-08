extern crate bitmex;
extern crate tokio;

use bitmex::model::instrument::GetInstrumentRequest;
use bitmex::{BitMEX, Result};
use tokio::runtime::Runtime;

#[test]
fn test_get_instrument() -> Result<()> {
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_instrument(GetInstrumentRequest {
        symbol: Some("XBT".to_string()),
        ..Default::default()
    })?;

    let res = rt.block_on(fut)?;
    println!("{:?}", res);
    Ok(())
}
