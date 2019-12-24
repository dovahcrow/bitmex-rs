use bitmex::models::GetLeaderboardRequest;
use bitmex::BitMEX;
use failure::Fallible;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_leaderboard() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.get_leaderboard(GetLeaderboardRequest {
        ..Default::default()
    })?;

    let _ = rt.block_on(fut)?;
    Ok(())
}

#[test]
fn get_leaderboard_name() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.get_leaderboard_name()?;

    let _ = rt.block_on(fut)?;
    Ok(())
}
