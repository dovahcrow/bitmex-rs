use failure::Error;
use futures::Future;

use error::Result;
use model::{Channel, Chat, ConnectedUsers};
use transport::Dummy;

use super::BitMEX;

impl BitMEX {
    pub fn get_chat<I, J, K>(&self, count: I, start: J, channel_id: K) -> Result<impl Future<Item = Vec<Chat>, Error = Error>>
    where
        I: Into<Option<usize>>,
        J: Into<Option<usize>>,
        K: Into<Option<usize>>,
    {
        let mut payload = vec![];
        if let Some(count) = count.into() {
            payload.push(("count", count.to_string()));
        }
        if let Some(start) = start.into() {
            payload.push(("start", start.to_string()));
        }
        if let Some(channel_id) = channel_id.into() {
            payload.push(("channelID", channel_id.to_string()));
        }

        Ok(self.transport.get("/chat", Some(payload))?)
    }

    pub fn post_chat<K>(&self, message: &str, channel_id: K) -> Result<impl Future<Item = Chat, Error = Error>>
    where
        K: Into<Option<usize>>,
    {
        let params = vec![("message", message.to_string()), ("channel_id", channel_id.into().unwrap_or(1).to_string())];
        Ok(self.transport.signed_post("/chat", Some(params))?)
    }

    pub fn channels(&self) -> Result<impl Future<Item = Vec<Channel>, Error = Error>> {
        Ok(self.transport.get::<_, Dummy, _, _>("/chat/channels", None)?)
    }

    pub fn connected_users(&self) -> Result<impl Future<Item = ConnectedUsers, Error = Error>> {
        Ok(self.transport.get::<_, Dummy, _, _>("/chat/connected", None)?)
    }
}
