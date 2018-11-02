extern crate bitmex;
extern crate dotenv;
extern crate tokio;

use std::env::var;

use bitmex::model::quote::{BinSize, GetQuoteBucketedRequest, GetQuoteRequest};
use bitmex::{BitMEX, Result};
use tokio::runtime::Runtime;

#[test]
fn get_quote() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    assert!(rt.block_on(bm.get_quote(GetQuoteRequest { ..Default::default() })?).is_err());
    Ok(())
}

#[test]
fn get_quote_bucketed() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    assert!(
        rt.block_on(bm.get_quote_bucketed(GetQuoteBucketedRequest {
            partial: false,
            bin_size: BinSize::D1,
            ..Default::default()
        })?).is_err()
    );
    Ok(())
}
