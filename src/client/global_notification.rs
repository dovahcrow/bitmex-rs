use failure::Error;
use futures::Future;

use error::Result;
use model::global_notification::GetGlobalNotificationResponse;

use super::BitMEX;

impl BitMEX {
    pub fn get_global_notification(&self) -> Result<impl Future<Item = Vec<GetGlobalNotificationResponse>, Error = Error>> {
        Ok(self.transport.signed_get::<_, ()>("/globalNotification", None)?)
    }
}
