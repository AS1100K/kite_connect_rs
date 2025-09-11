use std::time::Duration;

use reqwest::{
    Client, ClientBuilder,
    header::{HeaderMap, HeaderValue},
};
use serde::{Deserialize, Deserializer};

/// The default request timeout (in seconds) for all HTTP requests made by the client.
/// The default is 1 second.
///
/// If you want to update the request timeout for all requests, consider updating this
/// value before initializing [KiteConnect](super::KiteConnect).
pub static REQUEST_TIMEOUT_SECS: u64 = 1;

pub const API_VERSION: u8 = 3;
pub const API_VERSION_STR: &str = "3";

pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub struct AuthInfo {
    api_key: String,
    api_secret: String,
    access_token: String,
    /// Value of Authorization Header at each authenticated request
    authentication_header: String,
}

impl AuthInfo {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            api_key,
            api_secret,
            access_token: String::new(),
            authentication_header: String::new(),
        }
    }

    pub fn update_access_token(&mut self, access_token: String) {
        let authorization_header = format!("token {}:{access_token}", self.api_key);

        self.access_token = access_token;
        self.authentication_header = authorization_header;
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn api_secret(&self) -> &str {
        &self.api_secret
    }

    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn authentication_header(&self) -> &str {
        &self.authentication_header
    }
}

pub fn default_client_builder(
    authentication_header_value: Option<&str>,
) -> Result<Client, crate::Error> {
    let mut default_headers = HeaderMap::new();
    default_headers.insert("X-Kite-Version", HeaderValue::from_static(API_VERSION_STR));

    if let Some(authentication_header_value) = authentication_header_value {
        let mut auth_value = HeaderValue::from_str(authentication_header_value)?;
        auth_value.set_sensitive(true);
        default_headers.insert("Authorization", auth_value);
    }

    Ok(ClientBuilder::new()
        .default_headers(default_headers)
        .user_agent(APP_USER_AGENT)
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .build()?)
}

pub(crate) fn deserialize_nullable_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

pub(crate) fn deserialize_number_or_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{self, Visitor};
    use std::fmt;

    struct NumberOrStringVisitor;

    impl<'de> Visitor<'de> for NumberOrStringVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or a number")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_owned())
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }
    }

    deserializer.deserialize_any(NumberOrStringVisitor)
}
