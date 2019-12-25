use bitmex::models::GetFundingRequest;
use bitmex::BitMEX;
use failure::Fallible;
use log::debug;
use tokio::runtime::Runtime;

#[test]
fn get_funding() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();

    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.request(GetFundingRequest {
        symbol: Some("XBT".to_string()),
        ..Default::default()
    });

    debug!("{:?}", rt.block_on(fut)?);
    Ok(())
}
