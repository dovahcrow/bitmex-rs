use failure::Error;
use futures::Future;

use crate::error::Result;
use crate::model::execution::{GetExecutionRequest, GetExecutionResponse, GetExecutionTradeHistoryRequest, GetExecutionTradeHistoryResponse};
use crate::BitMEX;

impl BitMEX {
    pub fn get_execution(&self, req: GetExecutionRequest) -> Result<impl Future<Item = Vec<GetExecutionResponse>, Error = Error>> {
        Ok(self.transport.signed_get("/execution", Some(req))?)
    }

    pub fn get_execution_history(&self, req: GetExecutionTradeHistoryRequest) -> Result<impl Future<Item = Vec<GetExecutionTradeHistoryResponse>, Error = Error>> {
        Ok(self.transport.signed_get("/execution/tradeHistory", Some(req))?)
    }
}
