use failure::Error;
use futures::Future;

use crate::error::Result;
use crate::model::settlement::{GetSettlementRequest, GetSettlementResponse};
use crate::BitMEX;

impl BitMEX {
    pub fn get_settlement(&self, req: GetSettlementRequest) -> Result<impl Future<Item = GetSettlementResponse, Error = Error>> {
        Ok(self.transport.signed_get("/settlement", Some(req))?)
    }
}
