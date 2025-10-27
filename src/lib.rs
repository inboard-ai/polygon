//! Rust client library for polygon.io
//!
//! # Quick Start
//!
//! ```no_run
//! use polygon::Polygon;
//! use polygon::rest;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Polygon::default().with_key("your_api_key");
//!     let json = rest::aggs::previous_close(&client, "AAPL").get().await?;
//!     println!("{}", json);
//!     Ok(())
//! }
//! ```
//!
//! # Endpoint API
//!
//! Each endpoint returns a specific request builder type. Call `.get()` to execute:
//!
//! ```no_run
//! use polygon::Polygon;
//! use polygon::rest::tickers;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Polygon::default().with_key("your_api_key");
//!
//! // Raw JSON response
//! let json = tickers::related(&client, "AAPL").get().await?;
//!
//! // Decoded into typed structs
//! let data = tickers::all(&client)
//!     .limit(10)
//!     .exchange("XNYS")
//!     .decoded()
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
//!   Raw JSON endpoints in `rest::raw::*` (re-exported to `rest::*`) are always available regardless of this feature.
//!
//! - **`dotenvy`** - Enables loading API keys from environment variables via [`dotenvy`](https://docs.rs/dotenvy).
//!   Adds `Polygon::new()` which loads `POLYGON_API_KEY` from `.env` or environment.
//!   Without this feature, use `Polygon::default().with_key("your_key")` instead.
//!
//! - **`table`** - Enables Polars DataFrame output via [`polars`](https://docs.rs/polars).
//!   Provides `rest::table::*` modules that return DataFrames instead of JSON or structs.
//!
//! # LLM Tool Use
//!
//! Progressive discovery interface for AI agents:
//!
//! ```no_run
//! use polygon::Polygon;
//! use polygon::tool_use;
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Polygon::default().with_key("your_api_key");
//!
//! // Discover tools, modules, and endpoints
//! let tools = tool_use::call_tool(&client, json!({
//!     "tool": "list_tools",
//!     "params": {}
//! })).await?;
//!
//! // Get endpoint schema
//! let schema = tool_use::call_tool(&client, json!({
//!     "tool": "get_endpoint_schema",
//!     "params": { "module": "Aggs", "endpoint": "aggregates" }
//! })).await?;
//!
//! // Call endpoint
//! let data = tool_use::call_tool(&client, json!({
//!     "tool": "call_endpoint",
//!     "params": {
//!         "module": "Aggs",
//!         "endpoint": "aggregates",
//!         "arguments": {
//!             "ticker": "AAPL",
//!             "multiplier": 1,
//!             "timespan": "week",
//!             "from": "2024-01-01",
//!             "to": "2024-01-31"
//!         }
//!     }
//! })).await?;
//! # Ok(())
//! # }
//! ```
//!
//! See [`tool_use`] module for details.

#![warn(missing_docs)]

mod client;
pub mod error;
pub mod request;
pub mod response;
pub mod rest;

pub mod endpoint;
pub mod execute;
pub mod processor;
pub mod tool_use;

pub use error::{Error, Result};
pub use request::Request;
pub use response::Response;

/// The main polygon.io API client with the default `reqwest::Client` HTTP client.
///
/// This type alias is only available when the `reqwest` feature is enabled.
/// When `reqwest` is disabled, use `client::Polygon<YourClient>` directly.
#[cfg(feature = "reqwest")]
pub type Polygon = client::Polygon<reqwest::Client>;

// When reqwest is disabled, re-export the generic Polygon
#[cfg(not(feature = "reqwest"))]
pub use client::Polygon;

#[cfg(feature = "reqwest")]
static STATIC_INSTANCE: std::sync::LazyLock<arc_swap::ArcSwap<Polygon>> =
    std::sync::LazyLock::new(|| arc_swap::ArcSwap::from_pointee(Polygon::default()));

/// Initialize a static polygon instance.
#[cfg(feature = "reqwest")]
pub fn initialize(polygon: Polygon) -> std::sync::Arc<Polygon> {
    STATIC_INSTANCE.swap(std::sync::Arc::from(polygon))
}

/// Get the static polygon instance.
#[cfg(feature = "reqwest")]
pub fn instance() -> std::sync::Arc<Polygon> {
    STATIC_INSTANCE.load().clone()
}
