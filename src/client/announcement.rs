use failure::Error;
use futures::Future;

use error::Result;
use model::Announcement;
use transport::Dummy;

use super::BitMEX;

impl BitMEX {
    pub fn announcement(&self) -> Result<impl Future<Item = Vec<Announcement>, Error = Error>> {
        Ok(self.transport.get::<_, Dummy, _, _>("announcement", None)?)
    }

    pub fn announcement_urgent(&self) -> Result<impl Future<Item = Vec<Announcement>, Error = Error>> {
        Ok(self.transport.get::<_, Dummy, _, _>("announcement/urgent", None)?)
    }
}
