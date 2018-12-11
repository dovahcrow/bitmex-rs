use failure::Error;
use futures::Future;

use crate::error::Result;
use crate::model::funding::{GetFundingRequest, GetFundingResponse};
use crate::BitMEX;

impl BitMEX {
    pub fn get_funding(&self, req: GetFundingRequest) -> Result<impl Future<Item = Vec<GetFundingResponse>, Error = Error>> {
        Ok(self.transport.get("/funding", Some(req))?)
    }
}
