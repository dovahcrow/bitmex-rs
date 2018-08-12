use failure::Error;
use futures::Future;

use error::Result;
use model::{ApiKey, ApiKeyPermission, Success};
use transport::Dummy;

use super::BitMEX;

impl BitMEX {
    pub fn get_api_key(&self) -> Result<impl Future<Item = Vec<ApiKey>, Error = Error>> {
        Ok(self.transport.signed_get::<_, Dummy, _, _>("apiKey", None)?)
    }

    pub fn create_api_key(
        &self,
        name: &str,
        cidr: Option<&str>,
        permissions: ApiKeyPermission,
        enabled: bool,
        token: Option<&str>,
    ) -> Result<impl Future<Item = Vec<ApiKey>, Error = Error>> {
        let mut payload = vec![
            ("name", name.to_string()),
            ("cidr", cidr.unwrap_or_else(|| "0.0.0.0/0").to_string()),
            ("permissions", permissions.to_string()),
            ("enabled", enabled.to_string()),
        ];
        if let Some(token) = token {
            payload.push(("token", token.to_string()));
        }

        Ok(self.transport.signed_post("apiKey", Some(payload))?)
    }

    pub fn delete_api_key(&self, id: &str) -> Result<impl Future<Item = Success, Error = Error>> {
        Ok(self.transport.signed_delete("apiKey", Some(vec![("apiKeyID", id.to_string())]))?)
    }
    pub fn disable_api_key(&self, id: &str) -> Result<impl Future<Item = ApiKey, Error = Error>> {
        Ok(self.transport.signed_post("apiKey/disable", Some(vec![("apiKeyID", id.to_string())]))?)
    }
    pub fn enable_api_key(&self, id: &str) -> Result<impl Future<Item = ApiKey, Error = Error>> {
        Ok(self.transport.signed_post("apiKey/enable", Some(vec![("apiKeyID", id.to_string())]))?)
    }
}
