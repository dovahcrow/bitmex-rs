extern crate bitmex;
extern crate dotenv;
extern crate env_logger;
extern crate tokio;

use std::env::var;

use bitmex::model::order::{ContingencyType, DeleteOrderAllRequest, DeleteOrderRequest, ExecInst, OrdType, PostOrderRequest, PutOrderRequest, Side};

use bitmex::{BitMEX, Result};

use tokio::runtime::Runtime;

#[test]
fn create_order_market() -> Result<()> {
    ::dotenv::dotenv().ok();

    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let resp = rt.block_on(bm.post_order(PostOrderRequest {
        symbol: "XBTUSD".to_string(),
        order_qty: Some(1.),
        text: Some("Shine".into()),
        ..Default::default()
    })?)?;

    let _ = rt.block_on(bm.delete_order(DeleteOrderRequest {
        order_id: Some(resp.order_id),
        ..Default::default()
    })?)?;
    Ok(())
}

#[test]
fn create_order_limit_buy() -> Result<()> {
    ::dotenv::dotenv().ok();

    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let resp = rt.block_on(bm.post_order(PostOrderRequest {
        symbol: "XBTUSD".to_string(),
        ord_type: Some(OrdType::Limit),
        price: Some(6000.),
        order_qty: Some(1.),
        text: Some("Shine".into()),
        ..Default::default()
    })?)?;

    let _ = rt.block_on(bm.delete_order(DeleteOrderRequest {
        order_id: Some(resp.order_id),
        ..Default::default()
    })?)?;

    Ok(())
}

#[test]
fn create_order_limit_sell() -> Result<()> {
    ::dotenv::dotenv().ok();

    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let resp = rt.block_on(bm.post_order(PostOrderRequest {
        symbol: "XBTUSD".to_string(),
        ord_type: Some(OrdType::Limit),
        price: Some(6500.),
        order_qty: Some(-1.),
        text: Some("Shine".into()),
        ..Default::default()
    })?)?;

    let _ = rt.block_on(bm.delete_order(DeleteOrderRequest {
        order_id: Some(resp.order_id),
        ..Default::default()
    })?)?;

    Ok(())
}

#[test]
fn create_order_stop() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let resp = rt.block_on(bm.post_order(PostOrderRequest {
        symbol: "XBTUSD".to_string(),
        ord_type: Some(OrdType::Stop),
        stop_px: Some(7000.),
        order_qty: Some(-1.),
        text: Some("Shine".into()),
        ..Default::default()
    })?)?;

    let _ = rt.block_on(bm.delete_order(DeleteOrderRequest {
        order_id: Some(resp.order_id),
        ..Default::default()
    })?)?;

    Ok(())
}

#[test]
fn create_order_stoplimit() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let resp = rt.block_on(bm.post_order(PostOrderRequest {
        symbol: "XBTUSD".to_string(),
        ord_type: Some(OrdType::StopLimit),
        stop_px: Some(7000.),
        price: Some(7100.),
        order_qty: Some(-1.),
        text: Some("Shine".into()),
        ..Default::default()
    })?)?;

    let _ = rt.block_on(bm.delete_order(DeleteOrderRequest {
        order_id: Some(resp.order_id),
        ..Default::default()
    })?)?;

    Ok(())
}

#[test]
fn create_order_bracket() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let resp1 = rt.block_on(bm.post_order(PostOrderRequest {
        symbol: "XBTUSD".to_string(),
        cl_ord_link_id: Some("SHITTY".into()),
        ord_type: Some(OrdType::StopLimit),
        stop_px: Some(6000.),
        price: Some(6000.),
        order_qty: Some(1.),
        contingency_type: Some(ContingencyType::OneTriggersTheOther),
        text: Some("Entry".into()),
        ..Default::default()
    })?)?;

    let resp2 = rt.block_on(bm.post_order(PostOrderRequest {
        symbol: "XBTUSD".to_string(),
        side: Some(Side::Sell),
        cl_ord_link_id: Some("SHITTY".into()),
        ord_type: Some(OrdType::Stop),
        stop_px: Some(5900.),
        exec_inst: Some(vec![ExecInst::Close]),
        order_qty: Some(1.),
        text: Some("Stoploss".into()),
        ..Default::default()
    })?)?;

    let resp3 = rt.block_on(bm.post_order(PostOrderRequest {
        symbol: "XBTUSD".to_string(),
        cl_ord_link_id: Some("SHITTY".into()),
        ord_type: Some(OrdType::Limit),
        price: Some(7100.),
        order_qty: Some(-1.),
        exec_inst: Some(vec![ExecInst::Close]),
        text: Some("Profit".into()),
        ..Default::default()
    })?)?;

    let _ = rt.block_on(bm.delete_order(DeleteOrderRequest {
        order_id: Some(resp1.order_id),
        ..Default::default()
    })?)?;
    let _ = rt.block_on(bm.delete_order(DeleteOrderRequest {
        order_id: Some(resp2.order_id),
        ..Default::default()
    })?)?;
    let _ = rt.block_on(bm.delete_order(DeleteOrderRequest {
        order_id: Some(resp3.order_id),
        ..Default::default()
    })?)?;

    Ok(())
}

#[test]
fn create_amend_delete_order() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let cor = PostOrderRequest {
        symbol: "XBTUSD".to_string(),
        ord_type: Some(OrdType::StopLimit),
        stop_px: Some(6000.),
        price: Some(6000.),
        order_qty: Some(1000.),
        ..Default::default()
    };
    let fut = bm.post_order(cor)?;
    let resp = rt.block_on(fut)?;

    let fut = bm.put_order(PutOrderRequest {
        order_id: Some(resp.order_id),
        order_qty: Some(2.),
        ..Default::default()
    })?;
    let _ = rt.block_on(fut)?;

    let fut = bm.delete_order(DeleteOrderRequest {
        order_id: Some(resp.order_id),
        ..Default::default()
    })?;
    let _ = rt.block_on(fut)?;

    Ok(())
}

#[test]
fn create_delete_all_order() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let cor = PostOrderRequest {
        symbol: "XBTUSD".to_string(),
        ord_type: Some(OrdType::StopLimit),
        stop_px: Some(6000.),
        price: Some(6000.),
        order_qty: Some(20.),
        ..Default::default()
    };
    let fut = bm.post_order(cor)?;
    let _ = rt.block_on(fut)?;
    let cor = PostOrderRequest {
        symbol: "XBTUSD".to_string(),
        ord_type: Some(OrdType::StopLimit),
        stop_px: Some(6000.),
        price: Some(6000.),
        order_qty: Some(20.),
        ..Default::default()
    };
    let fut = bm.post_order(cor)?;
    let _ = rt.block_on(fut)?;

    let fut = bm.delete_order_all(DeleteOrderAllRequest {
        symbol: Some("XBTUSD".to_string()),
        ..Default::default()
    })?;
    let _ = rt.block_on(fut)?;

    Ok(())
}
