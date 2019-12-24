use bitmex::models::{
    GetUserWalletHistoryRequest, GetUserWalletRequest, GetUserWalletSummaryRequest,
};
use bitmex::BitMEX;
use failure::Fallible;
use failure::Fallible;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_user() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.request()?)?;
    Ok(())
}

#[test]
fn get_user_affiliate_status() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.request()?)?;
    Ok(())
}

#[test]
fn get_user_wallet() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.request(GetUserWalletRequest {
        ..Default::default()
    })?)?;

    Ok(())
}

#[test]
fn get_user_wallet_history() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.request(GetUserWalletHistoryRequest {
        ..Default::default()
    })?)?;

    Ok(())
}

#[test]
fn get_user_wallet_summary() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.request(GetUserWalletSummaryRequest {
        ..Default::default()
    })?)?;

    Ok(())
}

#[test]
fn get_user_commission() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.get_user_commission()?)?;

    Ok(())
}

#[test]
fn get_user_deposit_address() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.request(Default::default())?)?;

    Ok(())
}
