use failure::Error;
use futures::Future;

use error::Result;
use model::order::{
    DeleteOrderAllRequest, DeleteOrderAllResponse, DeleteOrderRequest, DeleteOrderResponse, GetOrderRequest, GetOrderResponse, PostOrderCancelAllAfterRequest,
    PostOrderCancelAllAfterResponse, PostOrderClosePositionRequest, PostOrderClosePositionResponse, PostOrderRequest, PostOrderResponse, PutOrderRequest, PutOrderResponse,
};

use super::BitMEX;

impl BitMEX {
    pub fn get_order(&self, req: GetOrderRequest) -> Result<impl Future<Item = GetOrderResponse, Error = Error>> {
        Ok(self.transport.signed_get("/order", Some(req))?)
    }

    pub fn put_order(&self, req: PutOrderRequest) -> Result<impl Future<Item = PutOrderResponse, Error = Error>> {
        Ok(self.transport.signed_put("/order", Some(req))?)
    }

    pub fn post_order(&self, req: PostOrderRequest) -> Result<impl Future<Item = PostOrderResponse, Error = Error>> {
        Ok(self.transport.signed_post("/order", Some(req))?)
    }

    pub fn delete_order(&self, req: DeleteOrderRequest) -> Result<impl Future<Item = Vec<DeleteOrderResponse>, Error = Error>> {
        Ok(self.transport.signed_delete("/order", Some(req))?)
    }

    pub fn delete_order_all(&self, req: DeleteOrderAllRequest) -> Result<impl Future<Item = Vec<DeleteOrderAllResponse>, Error = Error>> {
        Ok(self.transport.signed_delete("/order/all", Some(req))?)
    }

    pub fn post_order_cancel_all_after(&self, req: PostOrderCancelAllAfterRequest) -> Result<impl Future<Item = PostOrderCancelAllAfterResponse, Error = Error>> {
        Ok(self.transport.signed_post("/order/cancelAllAfter", Some(req))?)
    }

    pub fn post_order_close_position(&self, req: PostOrderClosePositionRequest) -> Result<impl Future<Item = PostOrderClosePositionResponse, Error = Error>> {
        Ok(self.transport.signed_post("/order/closePosition", Some(req))?)
    }
}
