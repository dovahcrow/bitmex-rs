use failure::Error;
use futures::Future;

use error::Result;
use model::position::{
    GetPositionRequest, GetPositionResponse, PostPositionIsolateRequest, PostPositionIsolateResponse, PostPositionLeverageRequest, PostPositionLeverageResponse,
    PostPositionRiskLimitRequest, PostPositionRiskLimitResponse, PostPositionTransferMarginRequest, PostPositionTransferMarginResponse,
};

use super::BitMEX;

impl BitMEX {
    pub fn get_position(&self, req: GetPositionRequest) -> Result<impl Future<Item = GetPositionResponse, Error = Error>> {
        Ok(self.transport.get("/position", Some(req))?)
    }
    pub fn post_position_isolate(&self, req: PostPositionIsolateRequest) -> Result<impl Future<Item = PostPositionIsolateResponse, Error = Error>> {
        Ok(self.transport.get("/position/isolate", Some(req))?)
    }
    pub fn post_position_leverage(&self, req: PostPositionLeverageRequest) -> Result<impl Future<Item = PostPositionLeverageResponse, Error = Error>> {
        Ok(self.transport.get("/position/leverage", Some(req))?)
    }
    pub fn post_position_risk_limit(&self, req: PostPositionRiskLimitRequest) -> Result<impl Future<Item = PostPositionRiskLimitResponse, Error = Error>> {
        Ok(self.transport.get("/position/riskLimit", Some(req))?)
    }
    pub fn post_position_transfer_margin(&self, req: PostPositionTransferMarginRequest) -> Result<impl Future<Item = PostPositionTransferMarginResponse, Error = Error>> {
        Ok(self.transport.get("/position/transferMargin", Some(req))?)
    }
}
