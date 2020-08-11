use bitmex::rest::BitMEXRest;
use bitmex::rest::{
    GetUserAffiliateStatusRequest, GetUserCommissionRequest, GetUserDepositAddressRequest,
    GetUserRequest, GetUserWalletHistoryRequest, GetUserWalletRequest, GetUserWalletSummaryRequest,
};
use failure::Fallible;
use log::debug;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_user() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;
    let bm = BitMEXRest::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.request(GetUserRequest))?;
    Ok(())
}

#[test]
fn get_user_affiliate_status() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;
    let bm = BitMEXRest::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.request(GetUserAffiliateStatusRequest);

    debug!("{:?}", rt.block_on(fut)?);
    Ok(())
}

#[test]
fn get_user_commission() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;
    let bm = BitMEXRest::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.request(GetUserCommissionRequest))?;

    Ok(())
}

#[test]
fn get_user_deposit_address() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;
    let bm = BitMEXRest::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.request(GetUserDepositAddressRequest {
        ..Default::default()
    }))?;

    Ok(())
}

#[test]
fn get_user_wallet() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;
    let bm = BitMEXRest::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.request(GetUserWalletRequest {
        ..Default::default()
    }))?;

    Ok(())
}

#[test]
fn get_user_wallet_history() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;
    let bm = BitMEXRest::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.request(GetUserWalletHistoryRequest {
        ..Default::default()
    }))?;

    Ok(())
}

#[test]
fn get_user_wallet_summary() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;
    let bm = BitMEXRest::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.request(GetUserWalletSummaryRequest {
        ..Default::default()
    }))?;

    Ok(())
}
