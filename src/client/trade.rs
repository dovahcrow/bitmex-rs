use failure::Error;
use futures::Future;

use crate::error::Result;
use crate::model::trade::{GetTradeBucketedRequest, GetTradeBucketedResponse, GetTradeRequest, GetTradeResponse};
use crate::BitMEX;

impl BitMEX {
    pub fn get_trade(&self, req: GetTradeRequest) -> Result<impl Future<Item = GetTradeResponse, Error = Error>> {
        Ok(self.transport.get("/trade", Some(req))?)
    }
    pub fn get_trade_bucketed(&self, req: GetTradeBucketedRequest) -> Result<impl Future<Item = GetTradeBucketedResponse, Error = Error>> {
        Ok(self.transport.get("/trade/bucketed", Some(req))?)
    }
}
