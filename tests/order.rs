use bitmex::models::{
    DeleteOrderAllRequest, DeleteOrderRequest, OrdType, PostOrderBulkRequest, PostOrderRequest,
    PutOrderBulkRequest, PutOrderRequest,
};
use bitmex::BitMEX;
use failure::Fallible;
use log::debug;
use serde_json::Value;
use std::env::var;
use tokio::runtime::Runtime;

// SKip order testings otherwise we will be marked as spammer
#[test]
fn create_order_market() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let resp = rt.block_on(bm.request(post_market_order(100)))?;

    let _ = rt.block_on(bm.request(DeleteOrderRequest {
        order_id: Some(Value::String(resp.order_id.to_string())),
        ..Default::default()
    }))?;
    Ok(())
}

#[test]
fn create_order_limit_buy() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let resp = rt.block_on(bm.request(post_limit_order(6000., 100)))?;

    let _ = rt.block_on(bm.request(DeleteOrderRequest {
        order_id: Some(Value::String(resp.order_id.to_string())),
        ..Default::default()
    }))?;

    Ok(())
}

#[test]
#[ignore]
fn create_order_limit_sell() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let resp = rt.block_on(bm.request(post_limit_order(6500., -100)))?;

    let _ = rt.block_on(bm.request(DeleteOrderRequest {
        order_id: Some(Value::String(resp.order_id.to_string())),
        ..Default::default()
    }))?;

    Ok(())
}

#[test]
fn create_order_stop() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let resp = rt.block_on(bm.request(post_stop_order(-100, 7000.)))?;

    let _ = rt.block_on(bm.request(DeleteOrderRequest {
        order_id: Some(Value::String(resp.order_id.to_string())),
        ..Default::default()
    }))?;

    Ok(())
}

#[test]
fn create_order_stoplimit() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let resp = rt.block_on(bm.request(post_stop_limit_order(7100., -100, 7000.)))?;

    let _ = rt.block_on(bm.request(DeleteOrderRequest {
        order_id: Some(Value::String(resp.order_id.to_string())),
        ..Default::default()
    }))?;

    Ok(())
}

#[test]
fn create_amend_delete_order() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let cor = post_stop_limit_order(6000., 1000, 6000.);
    let fut = bm.request(cor);
    let resp = rt.block_on(fut)?;

    let fut = bm.request(PutOrderRequest {
        order_id: Some(resp.order_id.to_string()),
        order_qty: Some(100),
        ..Default::default()
    });
    debug!("{:?}", rt.block_on(fut)?);

    let fut = bm.request(DeleteOrderRequest {
        order_id: Some(Value::String(resp.order_id.to_string())),
        ..Default::default()
    });
    debug!("{:?}", rt.block_on(fut)?);

    Ok(())
}

#[test]
fn create_delete_all_order() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let cor = post_stop_limit_order(6000., 200, 6000.);
    let fut = bm.request(cor);
    debug!("{:?}", rt.block_on(fut)?);
    let cor = post_stop_limit_order(6000., 200, 6000.);
    let fut = bm.request(cor);
    debug!("{:?}", rt.block_on(fut)?);

    let fut = bm.request(DeleteOrderAllRequest {
        symbol: Some("XBTUSD".to_string()),
        ..Default::default()
    });
    debug!("{:?}", rt.block_on(fut)?);

    Ok(())
}

#[test]
fn create_amend_delete_order_bulk() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);

    let orders = vec![
        post_stop_limit_order(6000., 100, 6000.),
        post_stop_limit_order(6000., 100, 6000.),
    ];

    let fut = bm.request(PostOrderBulkRequest {
        orders: Some(orders),
    });
    let orders = rt.block_on(fut)?;

    let req: Vec<_> = orders
        .into_iter()
        .map(|order| PutOrderRequest {
            order_id: Some(order.order_id.to_string()),
            order_qty: Some(110),
            ..Default::default()
        })
        .collect();

    let fut = bm.request(PutOrderBulkRequest { orders: Some(req) });
    debug!("{:?}", rt.block_on(fut));

    let fut = bm.request(DeleteOrderAllRequest {
        symbol: Some("XBTUSD".to_string()),
        filter: None,
        text: None,
    });
    debug!("{:?}", rt.block_on(fut)?);

    Ok(())
}

fn post_stop_limit_order(price: f64, qty: i32, stop: f64) -> PostOrderRequest {
    PostOrderRequest {
        symbol: "XBTUSD".to_string(),
        side: None,
        simple_order_qty: None,
        order_qty: Some(qty),
        price: Some(price),
        display_qty: None,
        stop_px: Some(stop),
        cl_ord_id: None,
        cl_ord_link_id: None,
        peg_offset_value: None,
        peg_price_type: None,
        ord_type: Some(OrdType::StopLimit),
        time_in_force: None,
        exec_inst: None,
        contingency_type: None,
        text: None,
    }
}

fn post_limit_order(price: f64, qty: i32) -> PostOrderRequest {
    PostOrderRequest {
        symbol: "XBTUSD".to_string(),
        side: None,
        simple_order_qty: None,
        order_qty: Some(qty),
        price: Some(price),
        display_qty: None,
        stop_px: None,
        cl_ord_id: None,
        cl_ord_link_id: None,
        peg_offset_value: None,
        peg_price_type: None,
        ord_type: Some(OrdType::Limit),
        time_in_force: None,
        exec_inst: None,
        contingency_type: None,
        text: Some("Shine".into()),
    }
}

fn post_stop_order(qty: i32, stop: f64) -> PostOrderRequest {
    PostOrderRequest {
        symbol: "XBTUSD".to_string(),
        side: None,
        simple_order_qty: None,
        order_qty: Some(qty),
        price: None,
        display_qty: None,
        stop_px: Some(stop),
        cl_ord_id: None,
        cl_ord_link_id: None,
        peg_offset_value: None,
        peg_price_type: None,
        ord_type: Some(OrdType::Stop),
        time_in_force: None,
        exec_inst: None,
        contingency_type: None,
        text: Some("Shine".into()),
    }
}

fn post_market_order(qty: i32) -> PostOrderRequest {
    PostOrderRequest {
        symbol: "XBTUSD".to_string(),
        side: None,
        simple_order_qty: None,
        order_qty: Some(qty),
        price: None,
        display_qty: None,
        stop_px: None,
        cl_ord_id: None,
        cl_ord_link_id: None,
        peg_offset_value: None,
        peg_price_type: None,
        ord_type: None,
        time_in_force: None,
        exec_inst: None,
        contingency_type: None,
        text: Some("Shine".into()),
    }
}
