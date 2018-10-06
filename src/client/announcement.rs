use failure::Error;
use futures::Future;

use error::Result;
use model::announcement::{GetAnnouncementRequest, GetAnnouncementResponse, GetAnnouncementUrgentResponse};

use super::BitMEX;

impl BitMEX {
    pub fn get_announcement(&self, req: GetAnnouncementRequest) -> Result<impl Future<Item = Vec<GetAnnouncementResponse>, Error = Error>> {
        Ok(self.transport.get("/announcement", Some(req))?)
    }

    pub fn get_announcement_urgent(&self) -> Result<impl Future<Item = Vec<GetAnnouncementUrgentResponse>, Error = Error>> {
        Ok(self.transport.get::<_, ()>("/announcement/urgent", None)?)
    }
}
