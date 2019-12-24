use bitmex::models::BinSize;
use bitmex::models::{GetTradeBucketedRequest, GetTradeRequest};
use bitmex::BitMEX;
use failure::Fallible;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_trade() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.get_trade(GetTradeRequest {
        ..Default::default()
    })?)?;
    Ok(())
}

#[test]
fn get_trade_bucketed() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.get_trade_bucketed(GetTradeBucketedRequest {
        partial: false,
        bin_size: BinSize::D1,
        count: 10,
        ..Default::default()
    })?)?;

    Ok(())
}
