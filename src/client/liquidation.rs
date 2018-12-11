use failure::Error;
use futures::Future;

use crate::error::Result;
use crate::model::liquidation::{GetLiquidationRequest, GetLiquidationResponse};
use crate::BitMEX;

impl BitMEX {
    pub fn get_liquidation(&self, req: GetLiquidationRequest) -> Result<impl Future<Item = Vec<GetLiquidationResponse>, Error = Error>> {
        Ok(self.transport.get("/liquidation", Some(req))?)
    }
}
