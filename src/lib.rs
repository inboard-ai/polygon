//! Rust client library for polygon.io
//!
//! This library provides an async client for interacting with the polygon.io API.
//!
//! # Features
//!
//! - `reqwest` (default): Uses `reqwest` as the HTTP client with a default type parameter
//! - `dotenvy` (default): Enables loading API keys from environment variables
//!
//! # Examples
//!
//! With both `reqwest` and `dotenvy` features enabled (default):
//!
//! ```ignore
//! use polygon::Polygon;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Polygon::new()?;
//!     let quotes = client.quotes();
//!     let quote = quotes.get_last_quote("AAPL").await?;
//!     Ok(())
//! }
//! ```
//!
//! With only `reqwest` feature enabled:
//!
//! ```ignore
//! use polygon::Polygon;
//!
//! let client = Polygon::default().with_key("my_api_key");
//! ```
//!
//! With only `dotenvy` feature enabled (custom HTTP client):
//!
//! ```ignore
//! use polygon::Polygon;
//! use polygon::Request as _;
//!
//! let http_client = MyHttpClient::new();
//! let client = Polygon::with_client(http_client)?;
//! ```

#![warn(missing_docs)]

mod client;
mod error;
mod request;
pub mod rest;
pub mod schema;

// Re-export main types
pub use error::{Error, Result};
pub use request::Request;

/// The main polygon.io API client with the default `reqwest::Client` HTTP client.
///
/// This type alias is only available when the `reqwest` feature is enabled.
/// When `reqwest` is disabled, use `client::Polygon<YourClient>` directly.
#[cfg(feature = "reqwest")]
pub type Polygon = client::Polygon<reqwest::Client>;

// When reqwest is disabled, re-export the generic Polygon
#[cfg(not(feature = "reqwest"))]
pub use client::Polygon;
