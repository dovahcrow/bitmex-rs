extern crate bitmex;
extern crate dotenv;
extern crate tokio;

use std::env::var;

use bitmex::model::order_book::GetOrderBookL2Request;
use bitmex::{BitMEX, Result};
use tokio::runtime::Runtime;

#[test]
fn get_order_book_l2() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.get_order_book_l2(GetOrderBookL2Request {
        symbol: "XBTUSD".into(),
        depth: Some(1),
    })?;

    let _ = rt.block_on(fut)?;
    Ok(())
}
