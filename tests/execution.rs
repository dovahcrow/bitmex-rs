extern crate bitmex;
extern crate dotenv;
extern crate tokio;

use std::env::var;

use bitmex::model::execution::{GetExecutionRequest, GetExecutionTradeHistoryRequest};
use bitmex::{BitMEX, Result};
use tokio::runtime::Runtime;

#[test]
fn get_execution() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.get_execution(GetExecutionRequest { ..Default::default() })?;

    let _ = rt.block_on(fut)?;
    Ok(())
}

#[test]
fn get_execution_history() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.get_execution_history(GetExecutionTradeHistoryRequest { ..Default::default() })?;

    let _ = rt.block_on(fut)?;
    Ok(())
}
