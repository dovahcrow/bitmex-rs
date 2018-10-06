use failure::Error;
use futures::Future;

use error::Result;
use model::order::{PostOrderRequest, PostOrderResponse, PutOrderRequest, PutOrderResponse};

use super::BitMEX;

impl BitMEX {
    pub fn post_order(&self, req: PostOrderRequest) -> Result<impl Future<Item = PostOrderResponse, Error = Error>> {
        Ok(self.transport.signed_post("/order", Some(req))?)
    }

    pub fn put_order(&self, req: PutOrderRequest) -> Result<impl Future<Item = PutOrderResponse, Error = Error>> {
        Ok(self.transport.signed_put("/order", Some(req))?)
    }
}
