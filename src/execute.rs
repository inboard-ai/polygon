//! Execute trait for running API requests
//!
//! The `Execute` trait provides the `.get()` method used by all endpoint builders
//! to execute requests and return results.
//!
//! # Example
//!
//! ```no_run
//! use polygon::Polygon;
//! use polygon::execute::Execute as _;
//! use polygon::rest::raw;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Polygon::default().with_key("your_api_key");
//! let json = raw::tickers::related(&client, "AAPL").get().await?;
//! # Ok(())
//! # }
//! ```

use crate::error::Result;

/// Trait for executing API requests
///
/// Implemented by all endpoint request builders (e.g., `Aggregates`, `News`, `All`).
/// Provides the `.get()` method to execute the request and return the result.
pub trait Execute {
    /// The output type of the request
    type Output;

    /// Execute the request and return the result
    fn get(self) -> impl std::future::Future<Output = Result<Self::Output>>;
}

