//! Ticker endpoint implementations

use crate::client::Polygon;
use crate::query::Query;
use crate::request::Request;

/// Get a list of all tickers
pub fn all<'a, Client: Request>(client: &'a Polygon<Client>) -> Query<'a, Client> {
    Query::new(client, "https://api.polygon.io/v3/reference/tickers")
}

/// Get detailed information about a ticker
pub fn details<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: &str,
) -> Query<'a, Client> {
    Query::new(
        client,
        format!("https://api.polygon.io/v3/reference/tickers/{}", ticker),
    )
}

/// Get tickers related to a given ticker
pub fn related<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: &str,
) -> Query<'a, Client> {
    Query::new(
        client,
        format!("https://api.polygon.io/v1/related-companies/{}", ticker),
    )
}

/// Get all ticker types
pub fn types<'a, Client: Request>(client: &'a Polygon<Client>) -> Query<'a, Client> {
    Query::new(client, "https://api.polygon.io/v3/reference/tickers/types")
}

/// Get event history for a ticker at a particular point in time
///
/// # Arguments
///
/// * `client` - Reference to the Polygon client
/// * `ticker` - The ticker symbol
///
/// # Example
///
/// ```no_run
/// use polygon::Polygon;
/// use polygon::rest::raw::tickers;
/// use polygon::query::Execute as _;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Polygon::default().with_key("your_api_key");
/// let data = tickers::events(&client, "AAPL").get().await?;
/// # Ok(())
/// # }
/// ```
pub fn events<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: &str,
) -> Query<'a, Client> {
    Query::new(
        client,
        format!("https://api.polygon.io/vX/reference/tickers/{}/events", ticker),
    )
    .optional("types")
}

/// Get the most recent news articles relating to a stock ticker symbol
///
/// # Arguments
///
/// * `client` - Reference to the Polygon client
///
/// # Example
///
/// ```no_run
/// use polygon::Polygon;
/// use polygon::rest::raw::tickers;
/// use polygon::query::Execute as _;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Polygon::default().with_key("your_api_key");
/// let data = tickers::news(&client)
///     .with("ticker", "AAPL")
///     .get().await?;
/// # Ok(())
/// # }
/// ```
pub fn news<'a, Client: Request>(client: &'a Polygon<Client>) -> Query<'a, Client> {
    Query::new(client, "https://api.polygon.io/v2/reference/news")
        .optional("ticker")
        .optional("ticker_lt")
        .optional("ticker_lte")
        .optional("ticker_gt")
        .optional("ticker_gte")
        .optional("published_utc")
        .optional("published_utc_lt")
        .optional("published_utc_lte")
        .optional("published_utc_gt")
        .optional("published_utc_gte")
        .optional("limit")
        .optional("sort")
        .optional("order")
}

#[cfg(all(test, feature = "dotenvy"))]
mod tests {
    use super::*;
    use crate::query::Execute as _;

    fn setup() -> Polygon<reqwest::Client> {
        Polygon::new().expect("Failed to create client. Make sure POLYGON_API_KEY is set in .env file")
    }

    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored --test-threads=1
    async fn test_all_tickers() {
        let client = setup();
        let result = all(&client).param("limit", "5").get().await;
        assert!(result.is_ok(), "Failed to fetch all tickers: {:?}", result);
    }

    #[tokio::test]
    #[ignore]
    async fn test_ticker_details() {
        let client = setup();
        let result = details(&client, "AAPL").get().await;
        assert!(result.is_ok(), "Failed to fetch ticker details: {:?}", result);
    }

    #[tokio::test]
    #[ignore]
    async fn test_related_tickers() {
        let client = setup();
        let result = related(&client, "AAPL").get().await;
        assert!(result.is_ok(), "Failed to fetch related tickers: {:?}", result);
    }

    #[tokio::test]
    #[ignore]
    async fn test_ticker_types() {
        let client = setup();
        let result = types(&client).get().await;
        assert!(result.is_ok(), "Failed to fetch ticker types: {:?}", result);
    }

    #[tokio::test]
    #[ignore]
    async fn test_ticker_events() {
        let client = setup();
        let result = events(&client, "AAPL").get().await;
        assert!(result.is_ok(), "Failed to fetch ticker events: {:?}", result);
    }
}
