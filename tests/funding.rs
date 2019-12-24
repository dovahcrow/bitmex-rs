use bitmex::models::GetFundingRequest;
use bitmex::BitMEX;
use failure::Fallible;
use tokio::runtime::Runtime;

#[test]
fn get_funding() -> Fallible<()> {
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
