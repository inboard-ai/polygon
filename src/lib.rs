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
//!     let client = Polygon::default().with_key("your_api_key");
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
//! let client = Polygon::default().with_key("your_api_key");
//!
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
//! - **`reqwest`** (default) - Uses [`reqwest`](https://docs.rs/reqwest) as the HTTP client.
//!   Disable this if you want to provide your own HTTP client implementation.
//!
//! - **`decoder`** (default) - Enables typed response decoding using the [`decoder`](https://docs.rs/decoder) crate.
//!   Provides `rest::tickers`, `rest::aggs`, etc. that return strongly-typed Rust structs.
//!   Raw JSON endpoints in `rest::raw::*` are always available regardless of this feature.
//!
//! - **`dotenvy`** - Enables loading API keys from environment variables via [`dotenvy`](https://docs.rs/dotenvy).
//!   Adds `Polygon::new()` which loads `POLYGON_API_KEY` from `.env` or environment.
//!   Without this feature, use `Polygon::default().with_key("your_key")` instead.

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
pub use request::{Request, Response};

/// The main polygon.io API client with the default `reqwest::Client` HTTP client.
///
/// This type alias is only available when the `reqwest` feature is enabled.
/// When `reqwest` is disabled, use `client::Polygon<YourClient>` directly.
#[cfg(feature = "reqwest")]
pub type Polygon = client::Polygon<reqwest::Client>;

// When reqwest is disabled, re-export the generic Polygon
#[cfg(not(feature = "reqwest"))]
pub use client::Polygon;
