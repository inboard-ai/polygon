//! Aggregates (bars) endpoint implementations returning raw JSON strings

use crate::client::Polygon;
use crate::processor::Raw;
use crate::request::Request;
use crate::request::aggs::{Aggregates, DailyOpenClose, GroupedDaily, PreviousClose};
use crate::request::common::Timespan;

/// Get aggregate bars for a stock over a given date range
///
/// Returns a request builder that will return results as raw JSON string.
/// Use builder methods like `.adjusted()`, `.sort()`, `.limit()` to customize the request.
///
/// # Example
/// ```no_run
/// # use polygon::Client;
/// # use polygon::execute::Execute;
/// # async fn example() {
/// # let client = Client::new("api-key");
/// let json = polygon::rest::aggs::aggregates(&client, "AAPL", 1, "day", "2023-01-01", "2023-12-31")
///     .adjusted(true)
///     .get()
///     .await
///     .unwrap();
/// # }
/// ```
pub fn aggregates<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: impl Into<String>,
    multiplier: u32,
    timespan: Timespan,
    from: impl Into<String>,
    to: impl Into<String>,
) -> Aggregates<'a, Client, Raw> {
    Aggregates::new(client, ticker, multiplier, timespan, from, to)
}

/// Get the previous day's open, high, low, and close (OHLC) for a stock
///
/// Returns a request builder that will return results as raw JSON string.
/// Use builder methods like `.adjusted()` to customize the request.
pub fn previous_close<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: impl Into<String>,
) -> PreviousClose<'a, Client, Raw> {
    PreviousClose::new(client, ticker)
}

/// Get the daily open, high, low, and close (OHLC) for the entire market
///
/// Returns a request builder that will return results as raw JSON string.
/// Use builder methods like `.adjusted()` and `.include_otc()` to customize the request.
pub fn grouped_daily<'a, Client: Request>(
    client: &'a Polygon<Client>,
    date: impl Into<String>,
) -> GroupedDaily<'a, Client, Raw> {
    GroupedDaily::new(client, date)
}

/// Get the open, close, and afterhours prices of a stock on a specific date
///
/// Returns a request builder that will return results as raw JSON string.
/// Use builder methods like `.adjusted()` to customize the request.
pub fn daily_open_close<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: impl Into<String>,
    date: impl Into<String>,
) -> DailyOpenClose<'a, Client, Raw> {
    DailyOpenClose::new(client, ticker, date)
}

#[cfg(all(test, feature = "dotenvy"))]
mod tests {
    use super::*;

    fn setup() -> Polygon<reqwest::Client> {
        Polygon::new().expect("Failed to create client. Make sure POLYGON_API_KEY is set in .env file")
    }

    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored --test-threads=1
    async fn test_aggregates() {
        let client = setup();
        let result = aggregates(&client, "AAPL", 1, Timespan::Day, "2023-01-01", "2023-01-10")
            .get()
            .await;
        assert!(result.is_ok(), "Failed to fetch aggregates: {result:?}");
    }

    #[tokio::test]
    #[ignore]
    async fn test_previous_close() {
        let client = setup();
        let result = previous_close(&client, "AAPL").get().await;
        assert!(result.is_ok(), "Failed to fetch previous close: {result:?}");
    }

    #[tokio::test]
    #[ignore]
    async fn test_grouped_daily() {
        let client = setup();
        let result = grouped_daily(&client, "2023-01-09").get().await;
        assert!(result.is_ok(), "Failed to fetch grouped daily: {result:?}");
    }

    #[tokio::test]
    #[ignore]
    async fn test_daily_open_close() {
        let client = setup();
        let result = daily_open_close(&client, "AAPL", "2023-01-09").get().await;
        assert!(result.is_ok(), "Failed to fetch daily open/close: {result:?}");
    }
}
