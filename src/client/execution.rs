use failure::Error;
use futures::Future;

use error::Result;
use model::execution::{GetExecutionRequest, GetExecutionResponse, GetExecutionTradeHistoryRequest, GetExecutionTradeHistoryResponse};

use super::BitMEX;

impl BitMEX {
    pub fn get_execution(&self, req: GetExecutionRequest) -> Result<impl Future<Item = Vec<GetExecutionResponse>, Error = Error>> {
        Ok(self.transport.get("/execution", Some(req))?)
    }

    pub fn get_execution_history(&self, req: GetExecutionTradeHistoryRequest) -> Result<impl Future<Item = Vec<GetExecutionTradeHistoryResponse>, Error = Error>> {
        Ok(self.transport.get::<_, ()>("/execution/tradeHistory", Some(req))?)
    }
}
