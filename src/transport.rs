use std::borrow::Borrow;

use chrono::{Duration, Utc};
use failure::Error;
use futures::{Future, Stream};
use hex::encode as hexify;
use hyper::client::{HttpConnector, ResponseFuture};
use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use ring::{digest, hmac};
use serde::de::DeserializeOwned;
use serde_json::from_slice;
use url::Url;

use error::{BitMEXError, BitMEXResponse, Result};

#[cfg(feature = "dev")]
const BASE: &'static str = "https://testnet.bitmex.com/api/v1";

#[cfg(not(feature = "dev"))]
const BASE: &'static str = "https://www.bitmex.com/api/v1";

pub struct Transport {
    client: Client<HttpsConnector<HttpConnector>>,
    credential: Option<(String, String)>,
}

impl Transport {
    pub fn new() -> Self {
        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder().build::<_, Body>(https);

        Transport { client: client, credential: None }
    }

    pub fn with_credential(api_key: &str, api_secret: &str) -> Self {
        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder().build::<_, Body>(https);

        Transport {
            client: client,
            credential: Some((api_key.into(), api_secret.into())),
        }
    }

    pub fn get<O: DeserializeOwned, I, K, V>(&self, endpoint: &str, params: Option<I>) -> Result<impl Future<Item = O, Error = Error>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let url = format!("{}/{}", BASE, endpoint);
        let url = match params {
            Some(p) => Url::parse_with_params(&url, p)?,
            None => Url::parse(&url)?,
        };
        Ok(self.handle_response(self.client.get(url.as_str().parse()?)))
    }

    // pub fn post<T>(&self, endpoint: &str) -> Result<impl Future> {}

    // fn put<T>(&self, endpoint: &str) -> Result<T> {}

    pub fn signed_get<O: DeserializeOwned, I, K, V>(&self, endpoint: &str, params: Option<I>) -> Result<impl Future<Item = O, Error = Error>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let url = format!("{}/{}", BASE, endpoint);
        let url = match params {
            Some(p) => Url::parse_with_params(&url, p)?,
            None => Url::parse(&url)?,
        };

        let expires = (Utc::now() + Duration::seconds(5)).timestamp();
        let (key, signature) = self.signature(Method::GET, expires, &url)?;

        let req = Request::get(url.as_str())
            .header("api-expires", expires)
            .header("api-key", key)
            .header("api-signature", signature)
            .body(Body::default())?;

        Ok(self.handle_response(self.client.request(req)))
    }

    fn check_key(&self) -> Result<(&str, &str)> {
        match self.credential.as_ref() {
            None => Err(BitMEXError::NoApiKeySet)?,
            Some((k, s)) => Ok((k, s)),
        }
    }

    fn signature(&self, method: Method, expires: i64, url: &Url) -> Result<(&str, String)> {
        let (key, secret) = self.check_key()?;
        // Signature: hex(HMAC_SHA256(apiSecret, verb + path + expires + data))
        let signed_key = hmac::SigningKey::new(&digest::SHA256, secret.as_bytes());
        let sign_message = match url.query() {
            Some(query) => format!("{}{}?{}{}", url, url.path(), query, expires),
            None => format!("{}{}{}", method.as_str(), url.path(), expires),
        };

        let signature = hexify(hmac::sign(&signed_key, sign_message.as_bytes()));
        Ok((key, signature))
    }

    fn handle_response<O: DeserializeOwned>(&self, fut: ResponseFuture) -> impl Future<Item = O, Error = Error> {
        fut.from_err::<Error>()
            .and_then(|resp| resp.into_body().concat2().from_err::<Error>())
            .map(|chunk| {
                trace!("{}", String::from_utf8_lossy(&*chunk));
                chunk
            })
            .and_then(|chunk| Ok(from_slice(&chunk)?))
            .and_then(|resp: BitMEXResponse<O>| Ok(resp.to_result()?))
    }
}
