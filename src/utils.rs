use std::time::Duration;

use reqwest::{
    Client, ClientBuilder,
    header::{HeaderMap, HeaderValue},
};
use serde::{Deserialize, Deserializer};

pub const REQUEST_TIMEOUT_SECS: u64 = 1;

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

pub fn deserialize_nullable_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
