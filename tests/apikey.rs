use std::env::var;

use bitmex::models::{
    DeleteApiKeyRequest, PostApiKeyDisableRequest, PostApiKeyEnableRequest, PostApiKeyRequest,
};
use bitmex::BitMEX;
use failure::Fallible;
use tokio::runtime::Runtime;

const TEST_KEY: &'static str = "kaiYN1s2-FIQzTxgHYisfovQ";
#[allow(dead_code)]
const TEST_SECRET: &'static str = "f6gkw2IfJgV62EWoxn_k7kj1SKpY9CBll_jJRmFI9AHjSlnL";

#[test]
fn get_api_key() -> Fallible<()> {
    ::dotenv::dotenv().ok();

    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.get_api_key()?;

    let _ = rt.block_on(fut)?;
    Ok(())
}

#[test]
fn create_api_key() -> Fallible<()> {
    ::dotenv::dotenv().ok();

    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.post_api_key(PostApiKeyRequest {
        ..Default::default()
    })?;

    assert!(rt.block_on(fut).is_err());
    Ok(())
}

#[test]
fn enable_api_key() -> Fallible<()> {
    ::dotenv::dotenv().ok();

    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.post_api_key_enable(PostApiKeyEnableRequest {
        api_key_id: TEST_KEY.into(),
    })?;

    assert!(rt.block_on(fut).is_err());
    Ok(())
}

#[test]

fn disable_api_key() -> Fallible<()> {
    ::dotenv::dotenv().ok();

    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.post_api_key_disable(PostApiKeyDisableRequest {
        api_key_id: TEST_KEY.into(),
    })?;

    assert!(rt.block_on(fut).is_err());
    Ok(())
}

#[test]

fn delete_api_key() -> Fallible<()> {
    ::dotenv::dotenv().ok();

    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.delete_api_key(DeleteApiKeyRequest {
        api_key_id: TEST_KEY.into(),
    })?;

    assert!(rt.block_on(fut).is_err());
    Ok(())
}
