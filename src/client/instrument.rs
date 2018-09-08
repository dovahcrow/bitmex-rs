use failure::Error;
use futures::Future;

use error::Result;
use model::instrument::{GetInstrumentRequest, GetInstrumentResponse};

use super::BitMEX;

impl BitMEX {
    pub fn get_instrument(&self, req: GetInstrumentRequest) -> Result<impl Future<Item = Vec<GetInstrumentResponse>, Error = Error>> {
        Ok(self.transport.get("/instrument", Some(req))?)
    }
}
