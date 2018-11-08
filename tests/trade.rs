extern crate bitmex;
extern crate dotenv;
extern crate tokio;

use std::env::var;

use bitmex::model::trade::{GetTradeBucketedRequest, GetTradeRequest};
use bitmex::model::BinSize;
use bitmex::{BitMEX, Result};
use tokio::runtime::Runtime;

#[test]
fn get_trade() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.get_trade(GetTradeRequest { ..Default::default() })?)?;
    Ok(())
}

#[test]
fn get_trade_bucketed() -> Result<()> {
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
