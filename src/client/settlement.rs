use failure::Error;
use futures::Future;

use error::Result;
use model::settlement::{GetSettlementRequest, GetSettlementResponse};

use super::BitMEX;

impl BitMEX {
    pub fn get_settlement(&self, req: GetSettlementRequest) -> Result<impl Future<Item = Vec<GetSettlementResponse>, Error = Error>> {
        Ok(self.transport.signed_get("/settlement", Some(req))?)
    }
}
