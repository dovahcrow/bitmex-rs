use failure::Error;
use futures::Future;

use crate::error::Result;
use crate::model::instrument::{
    GetInstrumentActiveAndIndicesResponse, GetInstrumentActiveIntervalsResponse, GetInstrumentActiveResponse, GetInstrumentCompositeIndexRequest,
    GetInstrumentCompositeIndexResponse, GetInstrumentIndicesResponse, GetInstrumentRequest, GetInstrumentResponse,
};
use crate::BitMEX;

impl BitMEX {
    pub fn get_instrument(&self, req: GetInstrumentRequest) -> Result<impl Future<Item = Vec<GetInstrumentResponse>, Error = Error>> {
        Ok(self.transport.get("/instrument", Some(req))?)
    }

    pub fn get_instrument_active(&self) -> Result<impl Future<Item = Vec<GetInstrumentActiveResponse>, Error = Error>> {
        Ok(self.transport.get::<_, ()>("/instrument/active", None)?)
    }

    pub fn get_instrument_active_and_indices(&self) -> Result<impl Future<Item = Vec<GetInstrumentActiveAndIndicesResponse>, Error = Error>> {
        Ok(self.transport.get::<_, ()>("/instrument/activeAndIndices", None)?)
    }

    pub fn get_instrument_active_interval(&self) -> Result<impl Future<Item = GetInstrumentActiveIntervalsResponse, Error = Error>> {
        Ok(self.transport.get::<_, ()>("/instrument/activeIntervals", None)?)
    }

    pub fn get_instrument_composite_index(&self, req: GetInstrumentCompositeIndexRequest) -> Result<impl Future<Item = Vec<GetInstrumentCompositeIndexResponse>, Error = Error>> {
        Ok(self.transport.get("/instrument/compositeIndex", Some(req))?)
    }

    pub fn get_instrument_indices(&self) -> Result<impl Future<Item = Vec<GetInstrumentIndicesResponse>, Error = Error>> {
        Ok(self.transport.get::<_, ()>("/instrument/indices", None)?)
    }
}
