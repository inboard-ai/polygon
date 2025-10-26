//! Ticker endpoint implementations

use crate::client::Polygon;
use crate::processor::Raw;
use crate::request::Request;
use crate::request::tickers::{All, Details, Events, News, Related, Types};

/// Get a list of all tickers
///
/// Returns a request builder that will return results as raw JSON string.
/// Use builder methods like `.ticker()`, `.ticker_type()`, `.market()`, `.limit()` to customize the request.
pub fn all<'a, Client: Request>(client: &'a Polygon<Client>) -> All<'a, Client, Raw> {
    All::new(client)
}

/// Get detailed information about a ticker
///
/// Returns a request builder that will return results as raw JSON string.
/// Use builder methods like `.date()` to customize the request.
pub fn details<'a, Client: Request>(client: &'a Polygon<Client>, ticker: &str) -> Details<'a, Client, Raw> {
    Details::new(client, ticker)
}

/// Get tickers related to a given ticker
///
/// Returns a request builder that will return results as raw JSON string.
pub fn related<'a, Client: Request>(client: &'a Polygon<Client>, ticker: &str) -> Related<'a, Client, Raw> {
    Related::new(client, ticker)
}

/// Get all ticker types
///
/// Returns a request builder that will return results as raw JSON string.
pub fn types<'a, Client: Request>(client: &'a Polygon<Client>) -> Types<'a, Client, Raw> {
    Types::new(client)
}

/// Get event history for a ticker at a particular point in time
///
/// Returns a request builder that will return results as raw JSON string.
/// Use builder methods like `.types()` to customize the request.
///
/// # Example
///
/// ```no_run
/// use polygon::Polygon;
/// use polygon::rest::raw::tickers;
/// use polygon::execute::Execute as _;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Polygon::default().with_key("your_api_key");
/// let data = tickers::events(&client, "AAPL").get().await?;
/// # Ok(())
/// # }
/// ```
pub fn events<'a, Client: Request>(client: &'a Polygon<Client>, ticker: &str) -> Events<'a, Client, Raw> {
    Events::new(client, ticker)
}

/// Get the most recent news articles relating to a stock ticker symbol
///
/// Returns a request builder that will return results as raw JSON string.
/// Use builder methods like `.ticker()`, `.limit()`, `.order()` to customize the request.
///
/// # Example
///
/// ```no_run
/// use polygon::Polygon;
/// use polygon::rest::raw::tickers;
/// use polygon::execute::Execute as _;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Polygon::default().with_key("your_api_key");
/// let data = tickers::news(&client)
///     .ticker("AAPL")
///     .get().await?;
/// # Ok(())
/// # }
/// ```
pub fn news<'a, Client: Request>(client: &'a Polygon<Client>) -> News<'a, Client, Raw> {
    News::new(client)
}

#[cfg(all(test, feature = "dotenvy"))]
mod tests {
    use super::*;

    fn setup() -> Polygon<reqwest::Client> {
        Polygon::new().expect("Failed to create client. Make sure POLYGON_API_KEY is set in .env file")
    }

    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored --test-threads=1
    async fn test_all_tickers() {
        let client = setup();
        let result = all(&client).limit("5").get().await;
        assert!(result.is_ok(), "Failed to fetch all tickers: {result:?}");
    }

    #[tokio::test]
    #[ignore]
    async fn test_ticker_details() {
        let client = setup();
        let result = details(&client, "AAPL").get().await;
        assert!(result.is_ok(), "Failed to fetch ticker details: {result:?}");
    }

    #[tokio::test]
    #[ignore]
    async fn test_related_tickers() {
        let client = setup();
        let result = related(&client, "AAPL").get().await;
        assert!(result.is_ok(), "Failed to fetch related tickers: {result:?}");
    }

    #[tokio::test]
    #[ignore]
    async fn test_ticker_types() {
        let client = setup();
        let result = types(&client).get().await;
        assert!(result.is_ok(), "Failed to fetch ticker types: {result:?}");
    }

    #[tokio::test]
    #[ignore]
    async fn test_ticker_events() {
        let client = setup();
        let result = events(&client, "AAPL").get().await;
        assert!(result.is_ok(), "Failed to fetch ticker events: {result:?}");
    }
}
