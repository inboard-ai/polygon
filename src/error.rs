//! Error types for the polygon.io API client

use std::error::Error as StdError;
use std::fmt;

/// Error type for polygon.io API operations
#[derive(Debug)]
pub enum Error {
    /// HTTP request error
    #[cfg(feature = "reqwest")]
    Reqwest(reqwest::Error),
    /// Environment variable error
    #[cfg(feature = "dotenvy")]
    Env(dotenvy::Error),
    /// Missing API key error
    MissingApiKey,
    /// Custom error message
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "reqwest")]
            Error::Reqwest(e) => write!(f, "HTTP request error: {}", e),
            #[cfg(feature = "dotenvy")]
            Error::Env(e) => write!(f, "Environment variable error: {}", e),
            Error::MissingApiKey => write!(f, "Missing API key"),
            Error::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            #[cfg(feature = "reqwest")]
            Error::Reqwest(e) => Some(e),
            #[cfg(feature = "dotenvy")]
            Error::Env(e) => Some(e),
            _ => None,
        }
    }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

#[cfg(feature = "dotenvy")]
impl From<dotenvy::Error> for Error {
    fn from(e: dotenvy::Error) -> Self {
        Error::Env(e)
    }
}

#[cfg(feature = "decoder")]
impl From<decoder::Error> for Error {
    fn from(e: decoder::Error) -> Self {
        Error::Custom(e.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Custom(e.to_string())
    }
}

/// Result type for polygon.io API operations
pub type Result<T> = std::result::Result<T, Error>;
