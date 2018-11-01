use failure::Error;
use futures::Future;

use error::Result;
use model::insurance::{GetInsuranceRequest, GetInsuranceResponse};

use super::BitMEX;

impl BitMEX {
    pub fn get_insurance(&self, req: GetInsuranceRequest) -> Result<impl Future<Item = Vec<GetInsuranceResponse>, Error = Error>> {
        Ok(self.transport.get("/insurance", Some(req))?)
    }
}
