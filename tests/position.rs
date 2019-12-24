use bitmex::models::{
    GetPositionRequest, PostPositionIsolateRequest, PostPositionLeverageRequest,
    PostPositionRiskLimitRequest, PostPositionTransferMarginRequest,
};
use bitmex::BitMEX;
use failure::Fallible;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_position() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let _ = ::env_logger::try_init();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.get_position(GetPositionRequest {
        ..Default::default()
    })?;

    let _ = rt.block_on(fut)?;
    Ok(())
}

#[test]
fn post_position_isolate() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let _ = ::env_logger::try_init();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.post_position_isolate(PostPositionIsolateRequest {
        symbol: "XBTUSD".into(),
        enabled: false,
    })?;

    let _ = rt.block_on(fut)?;
    Ok(())
}

#[test]
fn post_position_leverage() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let _ = ::env_logger::try_init();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.post_position_leverage(PostPositionLeverageRequest {
        symbol: "XBTUSD".into(),
        leverage: 1.1,
    })?;

    let _ = rt.block_on(fut)?;
    Ok(())
}

#[test]
fn post_position_risk_limit() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let _ = ::env_logger::try_init();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.post_position_risk_limit(PostPositionRiskLimitRequest {
        symbol: "XBTUSD".into(),
        risk_limit: 30000000000.0,
    })?;

    let a = rt.block_on(fut)?;
    println!("{:?}", a);
    Ok(())
}

#[test]
#[ignore]
fn post_position_transfer_margin() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let _ = ::env_logger::try_init();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.post_position_transfer_margin(PostPositionTransferMarginRequest {
        symbol: "XBTUSD".into(),
        amount: 10.,
    })?;

    let _ = rt.block_on(fut)?;
    Ok(())
}
