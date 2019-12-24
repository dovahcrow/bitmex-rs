use bitmex::models::GetSettlementRequest;
use bitmex::BitMEX;
use failure::Fallible;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_settlement() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    assert!(rt
        .block_on(bm.get_settlement(GetSettlementRequest {
            ..Default::default()
        })?)
        .is_err());
    Ok(())
}
