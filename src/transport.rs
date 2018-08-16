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
use serde::Serialize;
use serde_json::{from_slice, to_string, to_vec};
use url::Url;

use error::{BitMEXError, BitMEXResponse, Result};

#[cfg(feature = "dev")]
const BASE: &'static str = "https://testnet.bitmex.com/api/v1";

#[cfg(not(feature = "dev"))]
const BASE: &'static str = "https://www.bitmex.com/api/v1";

const EXPIRE_DURATION: i64 = 5;

pub(crate) type Dummy = &'static [(&'static str, &'static str); 0];

#[derive(Clone)]
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
        self.request::<_, _, _, _, ()>(Method::GET, endpoint, params, None)
    }

    pub fn signed_get<O, I, K, V>(&self, endpoint: &str, params: Option<I>) -> Result<impl Future<Item = O, Error = Error>>
    where
        O: DeserializeOwned,
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.signed_request::<_, _, _, _, ()>(Method::GET, endpoint, params, None)
    }

    pub fn signed_post<O, S>(&self, endpoint: &str, data: Option<S>) -> Result<impl Future<Item = O, Error = Error>>
    where
        O: DeserializeOwned,
        S: Serialize,
    {
        self.signed_request::<_, Dummy, _, _, S>(Method::POST, endpoint, None, data)
    }

    pub fn signed_put<O, S>(&self, endpoint: &str, data: Option<S>) -> Result<impl Future<Item = O, Error = Error>>
    where
        O: DeserializeOwned,
        S: Serialize,
    {
        self.signed_request::<_, Dummy, _, _, S>(Method::PUT, endpoint, None, data)
    }

    pub fn signed_delete<O: DeserializeOwned, I, K, V>(&self, endpoint: &str, params: Option<I>) -> Result<impl Future<Item = O, Error = Error>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.signed_request::<_, _, _, _, ()>(Method::DELETE, endpoint, params, None)
    }

    pub fn request<O: DeserializeOwned, I, K, V, S>(&self, method: Method, endpoint: &str, params: Option<I>, data: Option<S>) -> Result<impl Future<Item = O, Error = Error>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
        S: Serialize,
    {
        let url = format!("{}{}", BASE, endpoint);
        let url = match params {
            Some(p) => Url::parse_with_params(&url, p)?,
            None => Url::parse(&url)?,
        };

        let body = match data {
            Some(data) => Body::from(to_vec(&data)?),
            None => Body::empty(),
        };

        let req = Request::builder()
            .method(method)
            .uri(url.as_str())
            .header("user-agent", "bitmex-rs")
            .header("content-type", "application/json")
            .body(body)?;
        Ok(self.handle_response(self.client.request(req)))
    }

    pub fn signed_request<O: DeserializeOwned, I, K, V, S>(
        &self,
        method: Method,
        endpoint: &str,
        params: Option<I>,
        data: Option<S>,
    ) -> Result<impl Future<Item = O, Error = Error>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
        S: Serialize,
    {
        let url = format!("{}{}", BASE, endpoint);
        let url = match params {
            Some(p) => Url::parse_with_params(&url, p)?,
            None => Url::parse(&url)?,
        };

        let body = match data {
            Some(data) => to_string(&data)?,
            None => "".to_string(),
        };

        let expires = (Utc::now() + Duration::seconds(EXPIRE_DURATION)).timestamp();
        let (key, signature) = self.signature(method.clone(), expires, &url, &body)?;

        let req = Request::builder()
            .method(method)
            .uri(url.as_str())
            .header("api-expires", expires)
            .header("api-key", key)
            .header("api-signature", signature)
            .header("content-type", "application/json")
            .header("user-agent", "bitmex-rs")
            .body(Body::from(body))?;

        Ok(self.handle_response(self.client.request(req)))
    }

    fn check_key(&self) -> Result<(&str, &str)> {
        match self.credential.as_ref() {
            None => Err(BitMEXError::NoApiKeySet)?,
            Some((k, s)) => Ok((k, s)),
        }
    }

    pub(self) fn signature(&self, method: Method, expires: i64, url: &Url, body: &str) -> Result<(&str, String)> {
        let (key, secret) = self.check_key()?;
        // Signature: hex(HMAC_SHA256(apiSecret, verb + path + expires + data))
        let signed_key = hmac::SigningKey::new(&digest::SHA256, secret.as_bytes());
        let sign_message = match url.query() {
            Some(query) => format!("{}{}?{}{}{}", method.as_str(), url.path(), query, expires, body),
            None => format!("{}{}{}{}", method.as_str(), url.path(), expires, body),
        };
        trace!("Sign message {}", sign_message);
        let signature = hexify(hmac::sign(&signed_key, sign_message.as_bytes()));
        Ok((key, signature))
    }

    fn handle_response<O: DeserializeOwned>(&self, fut: ResponseFuture) -> impl Future<Item = O, Error = Error> {
        fut.from_err::<Error>()
            .and_then(|resp| resp.into_body().concat2().from_err::<Error>())
            .map(|chunk| {
                trace!("Response is {}", String::from_utf8_lossy(&*chunk));
                chunk
            })
            .and_then(|chunk| Ok(from_slice(&chunk)?))
            .and_then(|resp: BitMEXResponse<O>| Ok(resp.to_result()?))
    }
}

#[cfg(test)]
mod test {
    use super::Transport;
    use error::Result;
    use hyper::Method;
    use url::Url;

    #[test]
    fn test_signature_get() -> Result<()> {
        let tr = Transport::with_credential("LAqUlngMIQkIUjXMUreyu3qn", "chNOOS4KvNXR_Xq4k4c9qsfoKWvnDecLATCRlcBwyKDYnWgO");
        let (_, sig) = tr.signature(Method::GET, 1518064236, &Url::parse("http://a.com/api/v1/instrument")?, "")?;
        assert_eq!(sig, "c7682d435d0cfe87c16098df34ef2eb5a549d4c5a3c2b1f0f77b8af73423bf00");
        Ok(())
    }

    #[test]
    fn test_signature_get_param() -> Result<()> {
        let tr = Transport::with_credential("LAqUlngMIQkIUjXMUreyu3qn", "chNOOS4KvNXR_Xq4k4c9qsfoKWvnDecLATCRlcBwyKDYnWgO");
        let (_, sig) = tr.signature(
            Method::GET,
            1518064237,
            &Url::parse_with_params("http://a.com/api/v1/instrument", &[("filter", r#"{"symbol": "XBTM15"}"#)])?,
            "",
        )?;
        assert_eq!(sig, "e2f422547eecb5b3cb29ade2127e21b858b235b386bfa45e1c1756eb3383919f");
        Ok(())
    }

    #[test]
    fn test_signature_post() -> Result<()> {
        let tr = Transport::with_credential("LAqUlngMIQkIUjXMUreyu3qn", "chNOOS4KvNXR_Xq4k4c9qsfoKWvnDecLATCRlcBwyKDYnWgO");
        let (_, sig) = tr.signature(
            Method::POST,
            1518064238,
            &Url::parse("http://a.com/api/v1/order")?,
            r#"{"symbol":"XBTM15","price":219.0,"clOrdID":"mm_bitmex_1a/oemUeQ4CAJZgP3fjHsA","orderQty":98}"#,
        )?;
        assert_eq!(sig, "1749cd2ccae4aa49048ae09f0b95110cee706e0944e6a14ad0b3a8cb45bd336b");
        Ok(())
    }
}
