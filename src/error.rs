use std::fmt::Display;

/// Represents errors that can occur in this crate.
#[derive(Debug)]
pub enum Error {
    /// Error originating from the Kite API.
    KiteError(String),
    /// Error originating from serde_json serialization or deserialization.
    Serde(serde_json::Error),
    /// Error originating from reqwest HTTP requests.
    Reqwest(reqwest::Error),
    /// Error indicating that authentication has failed.
    AuthenticationFailed(String),
    /// Error indicating that the provided access token could not be converted to a header value.
    InvalidAccessToken,
    #[cfg(feature = "auto_auth")]
    IoError(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::KiteError(e) => write!(f, "Error originating from the Kite API. {e}"),
            Error::Serde(e) => write!(
                f,
                "Error originating from serde_json serialization or deserialization. {e}"
            ),
            Error::Reqwest(e) => write!(f, "Error originating from reqwest HTTP requests. {e}"),
            Error::AuthenticationFailed(msg) => {
                write!(f, "Error indicating that authentication has failed. {msg}")
            }
            Error::InvalidAccessToken => write!(
                f,
                "Error indicating that the provided access token could not be converted to a header value."
            ),
            #[cfg(feature = "auto_auth")]
            Error::IoError(e) => write!(f, "IO error: {e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(_value: reqwest::header::InvalidHeaderValue) -> Self {
        Self::InvalidAccessToken
    }
}

#[cfg(feature = "auto_auth")]
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
