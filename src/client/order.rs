use failure::Error;
use futures::Future;

use error::Result;
use model::Chat;
use transport::Dummy;

use super::BitMEX;

impl BitMEX {
    pub fn amend_order(&self) -> Result<impl Future<Item = Vec<Chat>, Error = Error>> {
        Ok(self.transport.signed_put::<_, Dummy, _, _>("/order", None)?)
    }
}
