use bitmex::rest::BitMEXRest;
use bitmex::rest::GetInsuranceRequest;
use failure::Fallible;
use log::debug;
use tokio::runtime::Runtime;

#[test]
fn get_insurance() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let rt = Runtime::new()?;

    let bm = BitMEXRest::new();
    let fut = bm.request(GetInsuranceRequest {
        ..Default::default()
    });

    debug!("{:?}", rt.block_on(fut)?);

    Ok(())
}
