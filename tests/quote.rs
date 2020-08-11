use bitmex::rest::BinSize;
use bitmex::rest::BitMEXRest;
use bitmex::rest::{GetQuoteBucketedRequest, GetQuoteRequest};
use failure::Fallible;
use log::debug;
use std::env::var;
use tokio::runtime::Runtime;

// 403 forbidden
#[test]
fn get_quote() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;
    let bm = BitMEXRest::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.request(GetQuoteRequest {
        ..Default::default()
    });
    let ret = rt.block_on(fut);
    debug!("{:?}", ret);

    assert!(ret.is_err());
    Ok(())
}

// 403 forbidden
#[test]
fn get_quote_bucketed() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;
    let bm = BitMEXRest::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.request(GetQuoteBucketedRequest {
        partial: Some(false),
        bin_size: Some(BinSize::D1),
        ..Default::default()
    });
    let ret = rt.block_on(fut);
    debug!("{:?}", ret);

    assert!(ret.is_err());
    Ok(())
}
