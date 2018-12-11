use failure::Error;
use futures::Future;

use crate::error::Result;
use crate::model::chat::{GetChatChannelsResponse, GetChatConnectedResponse, GetChatRequest, GetChatResponse, PostChatRequest, PostChatResponse};
use crate::BitMEX;

impl BitMEX {
    pub fn get_chat(&self, req: GetChatRequest) -> Result<impl Future<Item = Vec<GetChatResponse>, Error = Error>> {
        Ok(self.transport.get("/chat", Some(req))?)
    }

    pub fn post_chat(&self, req: PostChatRequest) -> Result<impl Future<Item = PostChatResponse, Error = Error>> {
        Ok(self.transport.signed_post("/chat", Some(req))?)
    }

    pub fn get_chat_channels(&self) -> Result<impl Future<Item = Vec<GetChatChannelsResponse>, Error = Error>> {
        Ok(self.transport.get::<_, ()>("/chat/channels", None)?)
    }

    pub fn get_chat_connected(&self) -> Result<impl Future<Item = GetChatConnectedResponse, Error = Error>> {
        Ok(self.transport.get::<_, ()>("/chat/connected", None)?)
    }
}
