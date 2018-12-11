use failure::Error;
use futures::Future;

use crate::error::Result;
use crate::model::position::{
    GetPositionRequest, GetPositionResponse, PostPositionIsolateRequest, PostPositionIsolateResponse, PostPositionLeverageRequest, PostPositionLeverageResponse,
    PostPositionRiskLimitRequest, PostPositionRiskLimitResponse, PostPositionTransferMarginRequest, PostPositionTransferMarginResponse,
};
use crate::BitMEX;

impl BitMEX {
    pub fn get_position(&self, req: GetPositionRequest) -> Result<impl Future<Item = Vec<GetPositionResponse>, Error = Error>> {
        Ok(self.transport.signed_get("/position", Some(req))?)
    }
    pub fn post_position_isolate(&self, req: PostPositionIsolateRequest) -> Result<impl Future<Item = PostPositionIsolateResponse, Error = Error>> {
        Ok(self.transport.signed_post("/position/isolate", Some(req))?)
    }
    pub fn post_position_leverage(&self, req: PostPositionLeverageRequest) -> Result<impl Future<Item = PostPositionLeverageResponse, Error = Error>> {
        Ok(self.transport.signed_post("/position/leverage", Some(req))?)
    }
    pub fn post_position_risk_limit(&self, req: PostPositionRiskLimitRequest) -> Result<impl Future<Item = PostPositionRiskLimitResponse, Error = Error>> {
        Ok(self.transport.signed_post("/position/riskLimit", Some(req))?)
    }
    pub fn post_position_transfer_margin(&self, req: PostPositionTransferMarginRequest) -> Result<impl Future<Item = PostPositionTransferMarginResponse, Error = Error>> {
        Ok(self.transport.signed_post("/position/transferMargin", Some(req))?)
    }
}
