// pub mod websocket;

use crate::models::Request;
use crate::transport::Transport;
use failure::Fallible;
use serde::de::DeserializeOwned;

#[derive(Clone)]
pub struct BitMEX {
    pub(crate) transport: Transport,
}

impl Default for BitMEX {
    fn default() -> Self {
        Self::new()
    }
}

impl BitMEX {
    pub fn new() -> Self {
        BitMEX {
            transport: Transport::new(),
        }
    }

    pub fn with_credential(api_key: &str, api_secret: &str) -> Self {
        BitMEX {
            transport: Transport::with_credential(api_key, api_secret),
        }
    }

    pub async fn request<R>(&self, req: R) -> Fallible<R::Response>
    where
        R: Request,
        R::Response: DeserializeOwned,
    {
        self.transport.request(req).await
    }
}
