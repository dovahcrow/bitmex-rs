use failure::Error;
use futures::Future;

use crate::error::Result;
use crate::model::user_event::{GetUserEventRequest, GetUserEventResponse};
use crate::BitMEX;

impl BitMEX {
    pub fn get_user_event(&self, req: GetUserEventRequest) -> Result<impl Future<Item = GetUserEventResponse, Error = Error>> {
        Ok(self.transport.signed_get("/userEvent", Some(req))?)
    }
}
