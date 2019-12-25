use bitmex::models::{
    GetInstrumentActiveAndIndicesRequest, GetInstrumentActiveIntervalsRequest,
    GetInstrumentActiveRequest, GetInstrumentCompositeIndexRequest, GetInstrumentIndicesRequest,
    GetInstrumentRequest,
};
use bitmex::BitMEX;
use failure::Fallible;
use log::debug;
use tokio::runtime::Runtime;

#[test]
fn test_get_instrument() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.request(GetInstrumentRequest {
        symbol: Some("XBT".to_string()),
        ..Default::default()
    });

    debug!("{:?}", rt.block_on(fut)?);
    Ok(())
}

#[test]
fn test_get_instrument_active() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.request(GetInstrumentActiveRequest {});

    debug!("{:?}", rt.block_on(fut)?);

    Ok(())
}

#[test]
fn test_get_instrument_active_and_indices() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.request(GetInstrumentActiveAndIndicesRequest {});

    debug!("{:?}", rt.block_on(fut)?);

    Ok(())
}

#[test]
fn get_instrument_active_interval() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.request(GetInstrumentActiveIntervalsRequest {});

    debug!("{:?}", rt.block_on(fut)?);

    Ok(())
}

#[test]
fn get_instrument_composite_index() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.request(GetInstrumentCompositeIndexRequest {
        symbol: Some("XBT".to_string()),
        ..Default::default()
    });

    debug!("{:?}", rt.block_on(fut)?);

    Ok(())
}

#[test]
fn get_instrument_indices() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.request(GetInstrumentIndicesRequest {});

    debug!("{:?}", rt.block_on(fut)?);

    Ok(())
}
