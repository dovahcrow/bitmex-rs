use failure::Error;
use futures::Future;

use error::Result;
use model::liquidation::{GetLiquidationRequest, GetLiquidationResponse};

use super::BitMEX;

impl BitMEX {
    pub fn get_liquidation(&self, req: GetLiquidationRequest) -> Result<impl Future<Item = Vec<GetLiquidationResponse>, Error = Error>> {
        Ok(self.transport.get("/liquidation", Some(req))?)
    }
}
