use failure::Error;
use futures::Future;

use crate::error::Result;
use crate::model::insurance::{GetInsuranceRequest, GetInsuranceResponse};
use crate::BitMEX;

impl BitMEX {
    pub fn get_insurance(&self, req: GetInsuranceRequest) -> Result<impl Future<Item = Vec<GetInsuranceResponse>, Error = Error>> {
        Ok(self.transport.get("/insurance", Some(req))?)
    }
}
