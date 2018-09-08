use failure::Error;
use futures::Future;

use error::Result;
use model::api_key::{
    DeleteApiKeyRequest, DeleteApiKeyResponse, GetApiKeyResponse, PostApiKeyDisableRequest, PostApiKeyDisableResponse, PostApiKeyEnableRequest, PostApiKeyEnableResponse,
    PostApiKeyRequest, PostApiKeyResponse,
};

use super::BitMEX;

impl BitMEX {
    pub fn get_api_key(&self) -> Result<impl Future<Item = Vec<GetApiKeyResponse>, Error = Error>> {
        Ok(self.transport.signed_get::<_, ()>("/apiKey", None)?)
    }

    pub fn create_api_key(&self, req: PostApiKeyRequest) -> Result<impl Future<Item = Vec<PostApiKeyResponse>, Error = Error>> {
        Ok(self.transport.signed_post("/apiKey", Some(req))?)
    }

    pub fn delete_api_key(&self, req: DeleteApiKeyRequest) -> Result<impl Future<Item = DeleteApiKeyResponse, Error = Error>> {
        Ok(self.transport.signed_delete("/apiKey", Some(req))?)
    }
    pub fn disable_api_key(&self, req: PostApiKeyDisableRequest) -> Result<impl Future<Item = PostApiKeyDisableResponse, Error = Error>> {
        Ok(self.transport.signed_post("/apiKey/disable", Some(req))?)
    }
    pub fn enable_api_key(&self, req: PostApiKeyEnableRequest) -> Result<impl Future<Item = PostApiKeyEnableResponse, Error = Error>> {
        Ok(self.transport.signed_post("/apiKey/enable", Some(req))?)
    }
}
