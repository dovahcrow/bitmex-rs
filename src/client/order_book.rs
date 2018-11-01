use failure::Error;
use futures::Future;

use error::Result;
use model::order_book::{GetOrderBookL2Request, GetOrderBookL2Response};

use super::BitMEX;

impl BitMEX {
    pub fn get_order_book_l2(&self, req: GetOrderBookL2Request) -> Result<impl Future<Item = Vec<GetOrderBookL2Response>, Error = Error>> {
        Ok(self.transport.get("/orderBook/L2", Some(req))?)
    }
}
