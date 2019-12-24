use bitmex::models::GetOrderBookL2Request;
use bitmex::BitMEX;
use failure::Fallible;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_order_book_l2() -> Fallible<()> {
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
