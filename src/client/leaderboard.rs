use failure::Error;
use futures::Future;

use error::Result;
use model::leaderboard::{GetLeaderboardNameResponse, GetLeaderboardRequest, GetLeaderboardResponse};

use super::BitMEX;

impl BitMEX {
    pub fn get_leaderboard(&self, req: GetLeaderboardRequest) -> Result<impl Future<Item = Vec<GetLeaderboardResponse>, Error = Error>> {
        Ok(self.transport.get("/leaderboard", Some(req))?)
    }

    pub fn get_leaderboard_name(&self) -> Result<impl Future<Item = GetLeaderboardNameResponse, Error = Error>> {
        Ok(self.transport.signed_get::<_, ()>("/leaderboard/name", None)?)
    }
}
