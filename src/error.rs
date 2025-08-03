use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt::Display};

#[derive(Debug, Serialize, Deserialize)]
pub enum KiteError {
    /// Preceded by a 403 header, this indicates the expiry or invalidation of an authenticated session.
    /// This can be caused by the user logging out, a natural expiry, or the user logging into another
    /// Kite instance. When you encounter this error, you should clear the user's session and
    /// re-initiate a login.
    TokenException(String),
    /// Represents user account related errors
    UserException(String),
    /// Represents order related errors such placement failures, a corrupt fetch etc
    OrderException(String),
    /// Represents missing required fields, bad values for parameters etc.
    InputException(String),
    /// Represents insufficient funds, required for the order placement
    MarginException(String),
    /// Represents insufficient holdings, available to place sell order for specified instrument
    HoldingException(String),
    /// Represents a network error where the API was unable to communicate with the OMS (Order Management System)
    NetworkException(String),
    /// Represents an internal system error where the API was unable to understand the response from the
    /// OMS to inturn respond to a request
    DataException(String),
    /// Represents an unclassified error. This should only happen rarely
    GeneralException(String),
    /// Unknown Error. `(error_type, message)`
    UnknownError(String, String),
}

impl From<(String, String)> for KiteError {
    fn from(value: (String, String)) -> Self {
        let (error_type, message) = value;
        let error_type = Cow::Owned(error_type);
        let message = Cow::Owned(message);

        (error_type, message).into()
    }
}

impl From<(&str, &str)> for KiteError {
    fn from(value: (&str, &str)) -> Self {
        let (error_type, message) = value;
        let error_type = Cow::Borrowed(error_type);
        let message = Cow::Borrowed(message);

        (error_type, message).into()
    }
}

impl From<(Cow<'_, str>, Cow<'_, str>)> for KiteError {
    fn from(value: (Cow<'_, str>, Cow<'_, str>)) -> Self {
        let (error_type, message) = value;

        match error_type.as_ref() {
            "TokenException" => Self::TokenException(message.into_owned()),
            "UserException" => Self::UserException(message.into_owned()),
            "OrderException" => Self::OrderException(message.into_owned()),
            "InputException" => Self::InputException(message.into_owned()),
            "MarginException" => Self::MarginException(message.into_owned()),
            "HoldingException" => Self::HoldingException(message.into_owned()),
            "NetworkException" => Self::NetworkException(message.into_owned()),
            "DataException" => Self::DataException(message.into_owned()),
            "GeneralException" => Self::GeneralException(message.into_owned()),
            _ => Self::UnknownError(error_type.into_owned(), message.into_owned()),
        }
    }
}

impl std::error::Error for KiteError {}

impl Display for KiteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KiteError::TokenException(message) => write!(f, "TokenException: {message}"),
            KiteError::UserException(message) => write!(f, "UserException: {message}"),
            KiteError::OrderException(message) => write!(f, "OrderException: {message}"),
            KiteError::InputException(message) => write!(f, "InputException: {message}"),
            KiteError::MarginException(message) => write!(f, "MarginException: {message}"),
            KiteError::HoldingException(message) => write!(f, "HoldingException: {message}"),
            KiteError::NetworkException(message) => {
                write!(f, "NetworkException: {message}")
            }
            KiteError::DataException(message) => write!(f, "DataException: {message}"),
            KiteError::GeneralException(message) => write!(f, "GeneralException: {message}"),
            KiteError::UnknownError(error_type, message) => {
                write!(f, "UnknownError: {error_type} ({message})")
            }
        }
    }
}

/// Represents errors that can occur in this crate.
#[derive(Debug)]
pub enum Error {
    /// Error originating from the Kite API.
    KiteError(KiteError),

    /// Error originating from serialization or deserialization.
    Serde(Box<dyn std::error::Error>),

    /// Error originating from reqwest HTTP requests.
    Reqwest(reqwest::Error),

    /// Error indicating that the provided access token could not be converted to a header value.
    InvalidAccessToken,

    /// Error related to IO
    #[cfg(feature = "auto_auth")]
    IoError(std::io::Error),

    /// Error indicating that the request timed out.
    RequestTimeOut,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::KiteError(e) => write!(f, "Error originating from the Kite API. {e}"),
            Error::Serde(e) => write!(
                f,
                "Error originating from serialization or deserialization. {e}"
            ),
            Error::Reqwest(e) => write!(f, "Error originating from reqwest HTTP requests. {e}"),
            Error::InvalidAccessToken => write!(
                f,
                "Error indicating that the provided access token could not be converted to a header value."
            ),
            #[cfg(feature = "auto_auth")]
            Error::IoError(e) => write!(f, "IO error: {e}"),
            Error::RequestTimeOut => write!(f, "Error indicating that the request timed out."),
        }
    }
}

impl std::error::Error for Error {}

impl From<KiteError> for Error {
    fn from(value: KiteError) -> Self {
        Self::KiteError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(Box::new(value))
    }
}

impl From<serde_urlencoded::ser::Error> for Error {
    fn from(value: serde_urlencoded::ser::Error) -> Self {
        Self::Serde(Box::new(value))
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        if value.is_timeout() {
            return Self::RequestTimeOut;
        }

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
