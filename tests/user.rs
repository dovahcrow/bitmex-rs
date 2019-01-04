extern crate bitmex;
extern crate dotenv;
extern crate tokio;

use std::env::var;

use bitmex::model::user::{GetUserWalletHistoryRequest, GetUserWalletRequest, GetUserWalletSummaryRequest};
use bitmex::{BitMEX, Result};
use tokio::runtime::Runtime;

#[test]
fn get_user() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.get_user()?)?;
    Ok(())
}


#[test]
fn get_user_affiliate_status() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.get_user_affiliate_status()?)?;
    Ok(())
}


#[test]
fn get_user_wallet() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.get_user_wallet(GetUserWalletRequest { ..Default::default() })?)?;

    Ok(())
}

#[test]
fn get_user_wallet_history() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.get_user_wallet_history(GetUserWalletHistoryRequest { ..Default::default() })?)?;

    Ok(())
}

#[test]
fn get_user_wallet_summary() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.get_user_wallet_summary(GetUserWalletSummaryRequest { ..Default::default() })?)?;

    Ok(())
}

#[test]
fn get_user_commission() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.get_user_commission()?)?;

    Ok(())
}

#[test]
fn get_user_deposit_address() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.get_user_deposit_address(Default::default())?)?;

    Ok(())
}

