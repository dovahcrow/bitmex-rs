use failure::Error;
use futures::Future;

use crate::error::Result;
use crate::model::quote::{GetQuoteBucketedRequest, GetQuoteBucketedResponse, GetQuoteRequest, GetQuoteResponse};
use crate::BitMEX;

impl BitMEX {
    pub fn get_quote(&self, req: GetQuoteRequest) -> Result<impl Future<Item = GetQuoteResponse, Error = Error>> {
        Ok(self.transport.get("/quote", Some(req))?)
    }
    pub fn get_quote_bucketed(&self, req: GetQuoteBucketedRequest) -> Result<impl Future<Item = GetQuoteBucketedResponse, Error = Error>> {
        Ok(self.transport.get("/quote/bucketed", Some(req))?)
    }
}
