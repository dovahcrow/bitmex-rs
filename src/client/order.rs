use failure::Error;
use futures::Future;

use error::Result;
use model::{AmendOrderRequest, AmendOrderResponse, CreateOrderRequest, CreateOrderResponse};

use super::BitMEX;

impl BitMEX {
    pub fn create_order(&self, req: CreateOrderRequest) -> Result<impl Future<Item = CreateOrderResponse, Error = Error>> {
        Ok(self.transport.signed_post("/order", Some(req))?)
    }

    pub fn amend_order(&self, req: AmendOrderRequest) -> Result<impl Future<Item = AmendOrderResponse, Error = Error>> {
        Ok(self.transport.signed_put("/order", Some(req))?)
    }
}
