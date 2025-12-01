//! Kite Connect API
//!
//! This crate provides a Rust client for the Kite Connect API v3, enabling developers to build
//! comprehensive investment and trading platforms. The API facilitates real-time order execution,
//! portfolio management, live market data streaming via WebSockets, and more.
//!
//! # Getting Started
//!
//! 1. Create a Kite Connect app at [https://developers.kite.trade/apps](https://developers.kite.trade/apps)
//! 2. Get your API key and API secret
//! 3. Initialize the client and authenticate
//!
//! # Example
//!
//! ```rust,no_run
//! use kite_connect::KiteConnect;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let api_key = "your_api_key".to_string();
//! let api_secret = "your_api_secret".to_string();
//! let kite = KiteConnect::new(api_key, api_secret);
//!
//! // Authenticate using request token from login flow
//! let request_token = "request_token_from_login";
//! let authenticated = kite.authenticate_with_request_token(request_token).await?;
//!
//! // Now you can use authenticated methods
//! let profile = authenticated.get_user_profile().await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Authentication
//!
//! The API uses OAuth 2.0 for authentication. Refer to the [official documentation](https://kite.trade/docs/connect/v3/user/)
//! for the complete authentication flow.
//!
//! # API Documentation
//!
//! For detailed API documentation, refer to:
//! - [Kite Connect HTTP API Documentation](https://kite.trade/docs/connect/v3/)

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
pub mod virtual_contract_note;
pub mod ws;

#[cfg(feature = "auto_auth")]
pub use auto_auth::AutoAuth;
pub use error::{Error, KiteError};
pub use response::Response;
pub use unimplemented::*;
pub use utils::{API_VERSION, REQUEST_TIMEOUT_SECS};

/// Marker type indicating that the `KiteConnect` instance is authenticated and ready to make API calls.
///
/// An authenticated instance has a valid access token and can perform all API operations.
pub struct Authenticated;

/// Marker type indicating that the `KiteConnect` instance is not yet authenticated.
///
/// An unauthenticated instance can only perform authentication operations.
pub struct AuthPending;

/// Trait for authentication status marker types.
///
/// This is a sealed trait that cannot be implemented outside this crate.
/// It is used to ensure type safety for authenticated vs unauthenticated states.
pub trait AuthStatus: sealed::Sealed {}

impl AuthStatus for Authenticated {}
impl AuthStatus for AuthPending {}

// TODO: Is this a good design decision?
mod sealed {
    pub trait Sealed {}
    impl Sealed for super::Authenticated {}
    impl Sealed for super::AuthPending {}
}

/// Main client for interacting with the Kite Connect API.
///
/// This struct is generic over an authentication status type parameter `T`, which can be either:
/// - `AuthPending`: The client is not authenticated and can only perform authentication operations
/// - `Authenticated`: The client is authenticated and can perform all API operations
///
/// # Type Safety
///
/// The type system ensures that only authenticated clients can call API methods that require
/// authentication. This prevents runtime errors from missing authentication.
///
/// # Example
///
/// ```rust,no_run
/// use kite_connect::KiteConnect;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Create an unauthenticated client
/// let kite = KiteConnect::new("api_key".to_string(), "api_secret".to_string());
///
/// // Authenticate
/// let authenticated = kite.authenticate_with_request_token("request_token").await?;
///
/// // Now authenticated methods are available
/// let profile = authenticated.get_user_profile().await?;
/// # Ok(())
/// # }
/// ```
///
/// # Thread Safety
///
/// `KiteConnect` implements `Clone`, allowing you to share the client across threads.
/// The underlying HTTP client is designed to be thread-safe.
#[derive(Clone)]
pub struct KiteConnect<T: AuthStatus = AuthPending> {
    pub(crate) client: Client,
    pub(crate) auth_info: AuthInfo,
    _auth_status: PhantomData<T>,
}

impl<T: AuthStatus> KiteConnect<T> {
    /// Returns a reference to the API key used by this `KiteConnect` instance.
    ///
    /// The API key is used for identifying your application when making API requests.
    ///
    /// # Returns
    ///
    /// A reference to the API key string.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use kite_connect::KiteConnect;
    /// let kite = KiteConnect::new("api_key".to_string(), "api_secret".to_string());
    /// let api_key = kite.api_key();
    /// ```
    #[inline]
    pub fn api_key(&self) -> &str {
        self.auth_info.api_key()
    }
}

impl KiteConnect<AuthPending> {
    /// Creates a new `KiteConnect` instance with the provided API credentials.
    ///
    /// This creates an unauthenticated client that can only perform authentication operations.
    /// To use API methods, you must first authenticate using either:
    /// - [`authenticate_with_request_token`](Self::authenticate_with_request_token) - for OAuth flow
    /// - [`authenticate_with_access_token`](Self::authenticate_with_access_token) - for existing tokens
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Kite Connect API key obtained from [Kite Connect Apps](https://developers.kite.trade/apps)
    /// * `api_secret` - Your Kite Connect API secret obtained from [Kite Connect Apps](https://developers.kite.trade/apps)
    ///
    /// # Returns
    ///
    /// A new `KiteConnect<AuthPending>` instance ready for authentication.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use kite_connect::KiteConnect;
    ///
    /// let kite = KiteConnect::new(
    ///     "your_api_key".to_string(),
    ///     "your_api_secret".to_string(),
    /// );
    /// ```
    ///
    /// # Panics
    ///
    /// This function will panic if the HTTP client cannot be initialized. This should only happen
    /// in exceptional circumstances.
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
    ///
    /// The access token is used for authenticating all API requests. It expires at 6 AM on the
    /// next day (regulatory requirement) unless invalidated earlier.
    ///
    /// # Returns
    ///
    /// A reference to the access token string.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use kite_connect::KiteConnect;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let kite = KiteConnect::new("api_key".to_string(), "api_secret".to_string());
    /// # let authenticated = kite.authenticate_with_request_token("token").await?;
    /// let access_token = authenticated.access_token();
    /// // You can store this token for later use
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn access_token(&self) -> &str {
        self.auth_info.access_token()
    }
}
