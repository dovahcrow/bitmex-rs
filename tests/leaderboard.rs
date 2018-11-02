extern crate bitmex;
extern crate dotenv;
extern crate tokio;

use std::env::var;

use bitmex::model::leaderboard::GetLeaderboardRequest;
use bitmex::{BitMEX, Result};
use tokio::runtime::Runtime;

#[test]
fn get_leaderboard() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.get_leaderboard(GetLeaderboardRequest { ..Default::default() })?;

    let _ = rt.block_on(fut)?;
    Ok(())
}

#[test]
fn get_leaderboard_name() -> Result<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.get_leaderboard_name()?;

    let _ = rt.block_on(fut)?;
    Ok(())
}
