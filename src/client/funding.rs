use failure::Error;
use futures::Future;

use error::Result;
use model::funding::{GetFundingRequest, GetFundingResponse};

use super::BitMEX;

impl BitMEX {
    pub fn get_funding(&self, req: GetFundingRequest) -> Result<impl Future<Item = Vec<GetFundingResponse>, Error = Error>> {
        Ok(self.transport.get("/funding", Some(req))?)
    }
}
