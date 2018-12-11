use failure::Error;
use futures::Future;

use crate::error::Result;
use crate::model::order_book::{GetOrderBookL2Request, GetOrderBookL2Response};
use crate::BitMEX;

impl BitMEX {
    pub fn get_order_book_l2(&self, req: GetOrderBookL2Request) -> Result<impl Future<Item = Vec<GetOrderBookL2Response>, Error = Error>> {
        Ok(self.transport.get("/orderBook/L2", Some(req))?)
    }
}
