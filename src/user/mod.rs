use serde::{Deserialize, Serialize};
use sha2::Digest;

use crate::{AuthPending, Authenticated, Error, KiteConnect, Response};

pub mod profile;
pub mod session_token;

pub const LOGIN_ENDPOINT: &str = "https://kite.zerodha.com/connect/login?v=3&api_key=";

/// Represents an exchange
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Exchange {
    /// BSE Futures & Options
    BFO,
    /// Multi Commodity Exchange
    MCX,
    /// National Stock Exchange
    NSE,
    /// Currency Derivatives Segment
    CDS,
    /// Bombay Stock Exchange
    BSE,
    /// Bombay Currency Derivatives
    BCD,
    /// Mutual Funds
    MF,
    /// NSE Futures & Options
    NFO,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Margin product types enabled for the user
pub enum Product {
    /// Cash and Carry
    CNC,
    /// Normal
    NRML,
    /// Margin Intraday Square-off
    MIS,
    /// Bracket Order
    BO,
    /// Cover Order
    CO,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Order types enabled for the user
pub enum OrderType {
    /// Market order
    #[serde(rename = "MARKET")]
    Market,
    /// Limit order
    #[serde(rename = "LIMIT")]
    Limit,
    /// Stop Loss order
    SL,
    /// Stop Loss Market order
    #[allow(non_camel_case_types)]
    #[serde(rename = "SL-M")]
    SL_M,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UserMetaData {
    pub demat_consent: DematConsent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DematConsent {
    #[default]
    Empty,
    Consent,
    Physical,
}

impl KiteConnect<AuthPending> {
    /// Authenticate using a `request_token` obtained from the Kite Connect login flow.
    ///
    /// This method exchanges the `request_token` for an `access_token` by calling the session token API.
    /// On success, it returns an authenticated `KiteConnect` instance.
    ///
    /// # Login Flow
    ///
    /// Refer to <https://kite.trade/docs/connect/v3/user/> for more information.
    ///
    /// # Arguments
    ///
    /// * `request_token` - The token received as a query parameter after a successful login.
    ///
    /// # Returns
    ///
    /// * `Ok(KiteConnect<Authenticated>)` if authentication succeeds.
    /// * `Err(Error)` if authentication fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kite_connect::KiteConnect;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let request_token = String::new();
    /// # let api_key = String::new();
    /// # let api_secret = String::new();
    /// let kite = KiteConnect::new(api_key, api_secret);
    /// let authenticated = kite.authenticate_with_request_token(&request_token).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn authenticate_with_request_token(
        mut self,
        request_token: &str,
    ) -> Result<KiteConnect<Authenticated>, Error> {
        let session_token = match self.generate_session_token(request_token).await? {
            Response::Success { data } => data,
            Response::Error { message, .. } => return Err(Error::AuthenticationFailed(message)),
        };

        self.auth_info
            .update_access_token(session_token.access_token);

        let client =
            crate::utils::default_client_builder(Some(self.auth_info.authentication_header()))?;

        Ok(KiteConnect {
            client,
            auth_info: self.auth_info,
            _auth_status: std::marker::PhantomData,
        })
    }

    /// Authenticate directly using an existing `access_token`.
    ///
    /// This method is useful if you have already obtained and persisted an `access_token` and want to reuse it.
    /// It does not perform any network requests.
    ///
    /// # Login Flow
    ///
    /// Refer to <https://kite.trade/docs/connect/v3/user/> for more information.
    ///
    /// # Arguments
    ///
    /// * `access_token` - The access token string to use for authentication.
    ///
    /// # Returns
    ///
    /// * `Ok(KiteConnect<Authenticated>)` if the token is set successfully.
    /// * `Err(Error)` if there is a problem setting up the client.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kite_connect::KiteConnect;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let api_key = String::new();
    /// # let api_secret = String::new();
    /// # let access_token = String::new();
    /// let kite = KiteConnect::new(api_key, api_secret);
    /// let authenticated = kite.authenticate_with_access_token(access_token)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn authenticate_with_access_token(
        mut self,
        access_token: String,
    ) -> Result<KiteConnect<Authenticated>, Error> {
        self.auth_info.update_access_token(access_token);

        let client =
            crate::utils::default_client_builder(Some(self.auth_info.authentication_header()))?;

        Ok(KiteConnect {
            client,
            auth_info: self.auth_info,
            _auth_status: std::marker::PhantomData,
        })
    }
}
