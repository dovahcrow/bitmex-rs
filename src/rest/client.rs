use super::{
    models::{swagger::SwaggerApiDescription, Request},
    BitMEXErrorResponse,
};
use crate::consts::REST_URL;
use crate::credential::Credential;
use crate::error::BitMEXError;
use crate::SWAGGER_URL;
use chrono::{Duration, Utc};
use derive_builder::Builder;
use fehler::{throw, throws};
use hyper::Method;
use log::error;
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use serde_json::{from_str, to_string as to_jstring};
use serde_urlencoded::to_string as to_ustring;
use url::Url;

const EXPIRE_DURATION: i64 = 5;

#[derive(Clone, Builder)]
pub struct BitMEXRest {
    client: Client,

    #[builder(default)]
    credential: Option<Credential>,
}

impl Default for BitMEXRest {
    fn default() -> Self {
        Self::new()
    }
}

impl BitMEXRest {
    pub fn new() -> Self {
        BitMEXRest {
            client: Client::new(),
            credential: None,
        }
    }

    pub fn with_credential(api_key: &str, api_secret: &str) -> Self {
        BitMEXRest {
            client: Client::new(),
            credential: Some(Credential(api_key.into(), api_secret.into())),
        }
    }

    pub fn builder() -> BitMEXRestBuilder {
        BitMEXRestBuilder::default()
    }

    #[throws(failure::Error)]
    pub async fn request<R>(&self, req: R) -> R::Response
    where
        R: Request,
        R::Response: DeserializeOwned,
    {
        let url = format!("{}{}", &*REST_URL, R::ENDPOINT);
        let mut url = Url::parse(&url)?;
        if matches!(R::METHOD, Method::GET | Method::DELETE) && R::HAS_PAYLOAD {
            url.set_query(Some(&to_ustring(&req)?));
        }

        let body = match R::METHOD {
            Method::PUT | Method::POST => to_jstring(&req)?,
            _ => "".to_string(),
        };

        let mut builder = self.client.request(R::METHOD, url.clone());

        if R::SIGNED {
            let cred = self.get_credential()?;
            let expires = (Utc::now() + Duration::seconds(EXPIRE_DURATION)).timestamp() as u64;
            let (key, signature) = cred.signature(R::METHOD, expires, &url, &body)?;

            builder = builder
                .header("api-expires", expires)
                .header("api-key", key)
                .header("api-signature", signature);
        }

        let resp = builder
            .header("content-type", "application/json")
            .header("user-agent", "bitmex-rs")
            .body(body)
            .send()
            .await?;

        self.handle_response(resp).await?
    }

    #[throws(failure::Error)]
    fn get_credential(&self) -> &Credential {
        match self.credential.as_ref() {
            None => throw!(BitMEXError::NoApiKeySet),
            Some(c) => c,
        }
    }

    #[throws(failure::Error)]
    async fn handle_response<T: DeserializeOwned>(&self, resp: Response) -> T {
        if resp.status().is_success() {
            let resp = resp.text().await?;
            match from_str::<T>(&resp) {
                Ok(resp) => resp,
                Err(e) => {
                    error!("Cannot deserialize '{}'", resp);
                    throw!(e);
                }
            }
        } else {
            let resp: BitMEXErrorResponse = resp.json().await?;
            throw!(BitMEXError::from(resp.error))
        }
    }

    #[throws(failure::Error)]
    pub async fn get_swagger(&self) -> SwaggerApiDescription {
        let resp: Response = self
            .client
            .get(SWAGGER_URL)
            .header("user-agent", "bitmex-rs")
            .header("content-type", "application/json")
            .send()
            .await?;
        self.handle_response(resp).await?
    }
}
