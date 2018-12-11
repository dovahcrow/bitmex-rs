use failure::Error;
use futures::Future;

use crate::error::Result;
use crate::model::global_notification::GetGlobalNotificationResponse;
use crate::BitMEX;

impl BitMEX {
    pub fn get_global_notification(&self) -> Result<impl Future<Item = Vec<GetGlobalNotificationResponse>, Error = Error>> {
        Ok(self.transport.signed_get::<_, ()>("/globalNotification", None)?)
    }
}
