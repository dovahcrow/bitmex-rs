use fehler::throws;
use hex::encode as hexify;
use http::Method;
use ring::hmac;
use url::Url;

#[derive(Clone, Debug)]
pub struct Credential(pub(crate) String, pub(crate) String);
impl Credential {
    #[throws(failure::Error)]
    pub(crate) fn signature(
        &self,
        method: Method,
        expires: u64,
        url: &Url,
        body: &str,
    ) -> (&str, String) {
        // Signature: hex(HMAC_SHA256(apiSecret, verb + path + expires + data))
        let signed_key = hmac::Key::new(hmac::HMAC_SHA256, self.1.as_bytes());
        let sign_message = match url.query() {
            Some(query) => format!(
                "{}{}?{}{}{}",
                method.as_str(),
                url.path(),
                query,
                expires,
                body
            ),
            None => format!("{}{}{}{}", method.as_str(), url.path(), expires, body),
        };

        let signature = hexify(hmac::sign(&signed_key, sign_message.as_bytes()));
        (self.0.as_str(), signature)
    }
}

#[cfg(test)]
mod test {
    use hyper::Method;
    use url::Url;

    use super::Credential;
    use failure::Fallible;

    #[test]
    fn test_signature_get() -> Fallible<()> {
        let tr = Credential(
            "LAqUlngMIQkIUjXMUreyu3qn".into(),
            "chNOOS4KvNXR_Xq4k4c9qsfoKWvnDecLATCRlcBwyKDYnWgO".into(),
        );
        let (_, sig) = tr.signature(
            Method::GET,
            1518064236,
            &Url::parse("http://a.com/api/v1/instrument")?,
            "",
        )?;
        assert_eq!(
            sig,
            "c7682d435d0cfe87c16098df34ef2eb5a549d4c5a3c2b1f0f77b8af73423bf00"
        );
        Ok(())
    }

    #[test]
    fn test_signature_get_param() -> Fallible<()> {
        let tr = Credential(
            "LAqUlngMIQkIUjXMUreyu3qn".into(),
            "chNOOS4KvNXR_Xq4k4c9qsfoKWvnDecLATCRlcBwyKDYnWgO".into(),
        );
        let (_, sig) = tr.signature(
            Method::GET,
            1518064237,
            &Url::parse_with_params(
                "http://a.com/api/v1/instrument",
                &[("filter", r#"{"symbol": "XBTM15"}"#)],
            )?,
            "",
        )?;
        assert_eq!(
            sig,
            "e2f422547eecb5b3cb29ade2127e21b858b235b386bfa45e1c1756eb3383919f"
        );
        Ok(())
    }

    #[test]
    fn test_signature_post() -> Fallible<()> {
        let tr = Credential(
            "LAqUlngMIQkIUjXMUreyu3qn".into(),
            "chNOOS4KvNXR_Xq4k4c9qsfoKWvnDecLATCRlcBwyKDYnWgO".into(),
        );
        let (_, sig) = tr.signature(
            Method::POST,
            1518064238,
            &Url::parse("http://a.com/api/v1/order")?,
            r#"{"symbol":"XBTM15","price":219.0,"clOrdID":"mm_bitmex_1a/oemUeQ4CAJZgP3fjHsA","orderQty":98}"#,
        )?;
        assert_eq!(
            sig,
            "1749cd2ccae4aa49048ae09f0b95110cee706e0944e6a14ad0b3a8cb45bd336b"
        );
        Ok(())
    }
}
