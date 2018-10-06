extern crate bitmex;
extern crate tokio;

use bitmex::model::instrument::GetInstrumentRequest;
use bitmex::{BitMEX, Result};
use tokio::runtime::Runtime;

#[test]
fn test_get_instrument() -> Result<()> {
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_instrument(GetInstrumentRequest {
        symbol: Some("XBT".to_string()),
        ..Default::default()
    })?;

    let res = rt.block_on(fut)?;
    println!("{:?}", res);
    Ok(())
}

#[test]
fn test_get_instrument_active() -> Result<()> {
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_instrument_active()?;

    let res = rt.block_on(fut)?;
    println!("{:?}", res);
    Ok(())
}

#[test]
fn test_get_instrument_active_and_indices() -> Result<()> {
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_instrument_active_and_indices()?;

    let res = rt.block_on(fut)?;
    println!("{:?}", res);
    Ok(())
}

#[test]
fn get_instrument_active_interval() -> Result<()> {
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_instrument_active_interval()?;

    let res = rt.block_on(fut)?;
    println!("{:?}", res);
    Ok(())
}

#[test]
fn get_instrument_composite_index() -> Result<()> {
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_instrument_composite_index(GetInstrumentRequest {
        symbol: Some("XBT".to_string()),
        ..Default::default()
    })?;

    let res = rt.block_on(fut)?;
    println!("{:?}", res);
    Ok(())
}

#[test]
fn get_instrument_indices() -> Result<()> {
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_instrument_indices()?;

    let res = rt.block_on(fut)?;
    println!("{:?}", res);
    Ok(())
}
