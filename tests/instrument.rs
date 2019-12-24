use bitmex::models::GetInstrumentRequest;
use bitmex::BitMEX;
use failure::Fallible;
use tokio::runtime::Runtime;

#[test]
fn test_get_instrument() -> Fallible<()> {
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
fn test_get_instrument_active() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_instrument_active()?;

    let _ = rt.block_on(fut)?;

    Ok(())
}

#[test]
fn test_get_instrument_active_and_indices() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_instrument_active_and_indices()?;

    let _ = rt.block_on(fut)?;

    Ok(())
}

#[test]
fn get_instrument_active_interval() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_instrument_active_interval()?;

    let _ = rt.block_on(fut)?;

    Ok(())
}

#[test]
fn get_instrument_composite_index() -> Fallible<()> {
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
fn get_instrument_indices() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.get_instrument_indices()?;

    let _ = rt.block_on(fut)?;

    Ok(())
}
