extern crate bitmex;
extern crate dotenv;
extern crate tokio;

use bitmex::model::instrument::GetInstrumentRequest;
use bitmex::{BitMEX, Result};
use tokio::runtime::Runtime;

#[test]
fn test_get_instrument() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_instrument(GetInstrumentRequest {
        symbol: Some("XBT".to_string()),
        ..Default::default()
    })?;

    let _ = rt.block_on(fut)?;
    Ok(())
}

#[test]
fn test_get_instrument_active() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_instrument_active()?;

    let _ = rt.block_on(fut)?;

    Ok(())
}

#[test]
fn test_get_instrument_active_and_indices() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_instrument_active_and_indices()?;

    let _ = rt.block_on(fut)?;

    Ok(())
}

#[test]
fn get_instrument_active_interval() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_instrument_active_interval()?;

    let _ = rt.block_on(fut)?;

    Ok(())
}

#[test]
fn get_instrument_composite_index() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_instrument_composite_index(GetInstrumentRequest {
        symbol: Some("XBT".to_string()),
        ..Default::default()
    })?;

    let _ = rt.block_on(fut)?;

    Ok(())
}

#[test]
fn get_instrument_indices() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_instrument_indices()?;

    let _ = rt.block_on(fut)?;

    Ok(())
}
