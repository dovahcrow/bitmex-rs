extern crate bitmex;
extern crate dotenv;
extern crate env_logger;
extern crate tokio;

use std::env::var;

use bitmex::model::{ContingencyType, CreateOrderRequest, ExecInst, OrdType, Side};
use bitmex::{BitMEX, Result};

use tokio::runtime::Runtime;

#[test]
fn create_order_market() -> Result<()> {
    ::dotenv::dotenv().ok();

    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let cor = CreateOrderRequest {
        symbol: "XBTUSD".to_string(),

        order_qty: Some(1.),
        text: Some("Shine".into()),
        ..Default::default()
    };
    let fut = bm.create_order(cor)?;

    let _ = rt.block_on(fut)?;
    Ok(())
}

#[test]
fn create_order_limit_buy() -> Result<()> {
    ::dotenv::dotenv().ok();

    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let cor = CreateOrderRequest {
        symbol: "XBTUSD".to_string(),
        ord_type: Some(OrdType::Limit),
        price: Some(6000.),
        order_qty: Some(1.),
        text: Some("Shine".into()),
        ..Default::default()
    };
    let fut = bm.create_order(cor)?;

    let _ = rt.block_on(fut)?;
    Ok(())
}

#[test]
fn create_order_limit_sell() -> Result<()> {
    ::dotenv::dotenv().ok();

    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let cor = CreateOrderRequest {
        symbol: "XBTUSD".to_string(),
        ord_type: Some(OrdType::Limit),
        price: Some(6000.),
        order_qty: Some(-1.),
        text: Some("Shine".into()),
        ..Default::default()
    };
    let fut = bm.create_order(cor)?;

    let _ = rt.block_on(fut)?;
    Ok(())
}

#[test]
fn create_order_stop() -> Result<()> {
    ::dotenv::dotenv().ok();

    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let cor = CreateOrderRequest {
        symbol: "XBTUSD".to_string(),
        ord_type: Some(OrdType::Stop),
        stop_px: Some(7000.),
        order_qty: Some(-1.),
        text: Some("Shine".into()),
        ..Default::default()
    };
    let fut = bm.create_order(cor)?;

    let _ = rt.block_on(fut)?;
    Ok(())
}

#[test]
fn create_order_stoplimit() -> Result<()> {
    ::dotenv::dotenv().ok();

    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let cor = CreateOrderRequest {
        symbol: "XBTUSD".to_string(),
        ord_type: Some(OrdType::StopLimit),
        stop_px: Some(7000.),
        price: Some(7100.),
        order_qty: Some(-1.),
        text: Some("Shine".into()),
        ..Default::default()
    };
    let fut = bm.create_order(cor)?;

    let _ = rt.block_on(fut)?;
    Ok(())
}

#[test]
fn create_order_bracket() -> Result<()> {
    ::dotenv::dotenv().ok();

    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let cor = CreateOrderRequest {
        symbol: "XBTUSD".to_string(),
        cl_ord_link_id: Some("SHITTY".into()),
        ord_type: Some(OrdType::StopLimit),
        stop_px: Some(6000.),
        price: Some(6000.),
        order_qty: Some(1.),
        contingency_type: Some(ContingencyType::OneTriggersTheOther),
        text: Some("Entry".into()),
        ..Default::default()
    };
    let fut = bm.create_order(cor)?;
    let _ = rt.block_on(fut)?;

    let cor = CreateOrderRequest {
        symbol: "XBTUSD".to_string(),
        side: Some(Side::Sell),
        cl_ord_link_id: Some("SHITTY".into()),
        ord_type: Some(OrdType::Stop),
        stop_px: Some(5900.),
        exec_inst: Some(ExecInst::Close),
        order_qty: Some(1.),
        text: Some("Stoploss".into()),
        ..Default::default()
    };
    let fut = bm.create_order(cor)?;
    let _ = rt.block_on(fut)?;

    let cor = CreateOrderRequest {
        symbol: "XBTUSD".to_string(),
        cl_ord_link_id: Some("SHITTY".into()),
        ord_type: Some(OrdType::Limit),
        price: Some(7100.),
        order_qty: Some(-1.),
        exec_inst: Some(ExecInst::Close),
        text: Some("Profit".into()),
        ..Default::default()
    };
    let fut = bm.create_order(cor)?;
    let _ = rt.block_on(fut)?;

    Ok(())
}
