use failure::Error;
use futures::Future;
use std::collections::BTreeMap;

use error::Result;
use model::{Announcement, ApiKey, Chat};
use transport::Transport;

type Dummy = &'static [(&'static str, &'static str); 0];

pub struct BitMEX {
    transport: Transport,
}

impl BitMEX {
    pub fn new() -> Self {
        BitMEX { transport: Transport::new() }
    }

    pub fn with_credential(api_key: &str, api_secret: &str) -> Self {
        BitMEX {
            transport: Transport::with_credential(api_key, api_secret),
        }
    }

    pub fn announcement(&self) -> Result<impl Future<Item = Vec<Announcement>, Error = Error>> {
        Ok(self.transport.get::<_, Dummy, _, _>("announcement", None)?)
    }

    pub fn announcement_urgent(&self) -> Result<impl Future<Item = Vec<Announcement>, Error = Error>> {
        Ok(self.transport.get::<_, Dummy, _, _>("announcement/urgent", None)?)
    }

    pub fn api_key(&self) -> Result<impl Future<Item = Vec<ApiKey>, Error = Error>> {
        Ok(self.transport.signed_get::<_, Dummy, _, _>("apiKey", None)?)
    }

    pub fn chat<I, J, K>(&self, count: I, start: J, channel_id: K) -> Result<impl Future<Item = Vec<Chat>, Error = Error>>
    where
        I: Into<Option<usize>>,
        J: Into<Option<usize>>,
        K: Into<Option<usize>>,
    {
        let mut bt = BTreeMap::new();
        if let Some(count) = count.into() {
            bt.insert("count", count.to_string());
        }
        if let Some(start) = start.into() {
            bt.insert("start", start.to_string());
        }
        if let Some(channel_id) = channel_id.into() {
            bt.insert("channelID", channel_id.to_string());
        }

        Ok(self.transport.get("chat", Some(bt))?)
    }
}
