use failure::Error;
use futures::Future;

use error::Result;
use model::Chat;

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

        Ok(self.transport.get("chat", Some(payload))?)
    }

    pub fn post_chat<K>(&self, message: &str, channel_id: K) -> Result<impl Future<Item = Chat, Error = Error>>
    where
        K: Into<Option<usize>>,
    {
        Ok(self.transport.signed_post(
            "chat",
            Some(vec![("message", message.to_string()), ("channel_id", channel_id.into().unwrap_or(1).to_string())]),
        )?)
    }
}
