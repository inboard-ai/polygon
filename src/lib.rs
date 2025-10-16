//! Rust client library for polygon.io
//!
//! # Quick Start
//!
//! ```no_run
//! use polygon::Polygon;
//! use polygon::rest::aggs;
//! use polygon::query::Execute as _;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Polygon::new()?;
//!     let result = aggs::previous_close(&client, "AAPL").get().await?;
//!     println!("{:?}", result);
//!     Ok(())
//! }
//! ```
//!
//! # Query API
//!
//! Endpoints return a `Query` builder. Call `.get()` to execute:
//!
//! ```no_run
//! use polygon::Polygon;
//! use polygon::query::Execute as _;
//! use polygon::rest::{raw, tickers};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Polygon::new()?;
//! // Raw JSON response
//! let json = raw::tickers::related(&client, "AAPL").get().await?;
//!
//! // Decoded into typed structs
//! let data = tickers::all(&client)
//!     .param("limit", 10)
//!     .params([("exchange", "XNYS"), ("sort", "ticker")])
//!     .get()
//!     .await?;
//!
//! println!("{:?} {:?}", data[0].ticker, data[0].name);
//! # Ok(())
//! # }
//! ```
//!
//! # Features
//!
//! - `reqwest` (default): Uses `reqwest` as the HTTP client
//! - `dotenvy` (default): Enables loading API keys from environment variables
//! - `decoder` (default): Enables typed response decoding

#![warn(missing_docs)]

mod client;
mod error;
mod request;
pub mod rest;
pub mod schema;

#[cfg(feature = "decoder")]
pub mod query;

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
