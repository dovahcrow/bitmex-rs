use bitmex::models::BinSize;
use bitmex::models::{GetQuoteBucketedRequest, GetQuoteRequest};
use bitmex::BitMEX;
use failure::Fallible;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_quote() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    assert!(rt
        .block_on(bm.get_quote(GetQuoteRequest {
            ..Default::default()
        })?)
        .is_err());
    Ok(())
}

#[test]
fn get_quote_bucketed() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    assert!(rt
        .block_on(bm.get_quote_bucketed(GetQuoteBucketedRequest {
            partial: false,
            bin_size: BinSize::D1,
            ..Default::default()
        })?)
        .is_err());
    Ok(())
}
