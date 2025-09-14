//! Kite Connect API

use reqwest::Client;
use std::marker::PhantomData;
use utils::AuthInfo;

#[cfg(feature = "auto_auth")]
mod auto_auth;
mod error;
pub mod historical;
pub mod orders;
pub mod portfolio;
pub mod quotes;
mod response;
mod unimplemented;
pub mod user;
pub(crate) mod utils;

#[cfg(feature = "auto_auth")]
pub use auto_auth::AutoAuth;
pub use error::{Error, KiteError};
pub use response::Response;
pub use unimplemented::*;
pub use utils::{API_VERSION, REQUEST_TIMEOUT_SECS};

pub struct Authenticated;
pub struct AuthPending;

pub trait AuthStatus: sealed::Sealed {}

impl AuthStatus for Authenticated {}
impl AuthStatus for AuthPending {}

// TODO: Is this a good design decision?
mod sealed {
    pub trait Sealed {}
    impl Sealed for super::Authenticated {}
    impl Sealed for super::AuthPending {}
}

pub struct KiteConnect<T: AuthStatus = AuthPending> {
    pub(crate) client: Client,
    pub(crate) auth_info: AuthInfo,
    _auth_status: PhantomData<T>,
}

impl<T: AuthStatus> KiteConnect<T> {
    /// Returns a reference to the API key used by this `KiteConnect` instance.
    #[inline]
    pub fn api_key(&self) -> &str {
        self.auth_info.api_key()
    }
}

impl KiteConnect<AuthPending> {
    pub fn new(api_key: String, api_secret: String) -> Self {
        let client = utils::default_client_builder(None).expect("Error in default_client_builder");

        Self {
            client,
            auth_info: AuthInfo::new(api_key, api_secret),
            _auth_status: PhantomData,
        }
    }
}

impl KiteConnect<Authenticated> {
    /// Returns a reference to the access token used by this `KiteConnect` instance.
    #[inline]
    pub fn access_token(&self) -> &str {
        self.auth_info.access_token()
    }
}
