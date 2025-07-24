use reqwest::{
    Client, ClientBuilder,
    header::{HeaderMap, HeaderValue},
};

pub const API_VERSION: u8 = 3;
pub const API_VERSION_STR: &str = "3";

pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub struct AuthInfo {
    api_key: String,
    api_secret: String,
    access_token: String,
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
        let authorization_header = format!("{}:{}", self.api_key, access_token);

        self.access_token = access_token;
        self.authentication_header = authorization_header;
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn api_secret(&self) -> &str {
        &self.api_secret
    }

    pub fn authentication_header(&self) -> &str {
        &self.authentication_header
    }
}

pub fn default_client_builder(
    authentication_header_value: Option<&str>,
) -> Result<Client, crate::error::Error> {
    let mut default_headers = HeaderMap::new();
    default_headers.insert("X-Kite-Version", HeaderValue::from_static(API_VERSION_STR));

    if let Some(authentication_header_value) = authentication_header_value {
        let mut auth_value = HeaderValue::from_str(authentication_header_value)?;
        auth_value.set_sensitive(true);
        default_headers.insert("Authentication", auth_value);
    }

    Ok(ClientBuilder::new()
        .default_headers(default_headers)
        .user_agent(APP_USER_AGENT)
        .build()?)
}
