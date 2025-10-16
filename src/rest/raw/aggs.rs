//! Aggregates (bars) endpoint implementations

use crate::client::Polygon;
use crate::query::Query;
use crate::request::Request;

/// Get aggregate bars for a stock over a given date range
///
/// # Arguments
///
/// * `client` - Reference to the Polygon client
/// * `ticker` - The ticker symbol
/// * `multiplier` - The size of the timespan multiplier (e.g., 1)
/// * `timespan` - The size of the time window (e.g., "minute", "hour", "day", "week", "month", "quarter", "year")
/// * `from` - The start of the aggregate time window (YYYY-MM-DD or Unix MS timestamp)
/// * `to` - The end of the aggregate time window (YYYY-MM-DD or Unix MS timestamp)
/// * `adjusted` - Whether results are adjusted for splits (default: true)
/// * `sort` - Sort order: "asc" or "desc" (default: "asc")
/// * `limit` - Limit the number of results (max 50000, default 5000)
///
/// # Example
///
/// ```ignore
/// use polygon::{Polygon, rest::aggs};
///
/// let client = Polygon::new()?;
/// let data = aggs::aggregates(&client, "AAPL", 1, "day", "2023-01-01", "2023-01-31", None, None, None).await?;
/// ```
pub fn aggregates<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: &str,
    multiplier: u32,
    timespan: &str,
    from: &str,
    to: &str,
) -> Query<'a, Client> {
    Query::new(
        client,
        format!(
            "https://api.polygon.io/v2/aggs/ticker/{}/range/{}/{}/{}/{}",
            ticker, multiplier, timespan, from, to
        ),
    )
    .optional("adjusted")
    .optional("sort")
    .optional("limit")
}

/// Get the previous day's open, high, low, and close (OHLC) for a stock
///
/// # Arguments
///
/// * `client` - Reference to the Polygon client
/// * `ticker` - The ticker symbol
///
/// # Example
///
/// ```ignore
/// use polygon::{Polygon, rest::aggs};
///
/// let client = Polygon::new()?;
/// let data = aggs::previous_close(&client, "AAPL").await?;
/// ```
pub fn previous_close<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: &str,
) -> Query<'a, Client> {
    Query::new(client, format!("https://api.polygon.io/v2/aggs/ticker/{}/prev", ticker))
        .optional("adjusted")
}

/// Get the daily open, high, low, and close (OHLC) for the entire market
///
/// # Arguments
///
/// * `client` - Reference to the Polygon client
/// * `date` - The date for the aggregate window (YYYY-MM-DD)
///
/// # Example
///
/// ```ignore
/// use polygon::{Polygon, rest::aggs};
///
/// let client = Polygon::new()?;
/// let data = aggs::grouped_daily(&client, "2023-01-09").await?;
/// ```
pub fn grouped_daily<'a, Client: Request>(
    client: &'a Polygon<Client>,
    date: &str,
) -> Query<'a, Client> {
    Query::new(
        client,
        format!("https://api.polygon.io/v2/aggs/grouped/locale/us/market/stocks/{}", date),
    )
    .optional("adjusted")
    .optional("include_otc")
}

/// Get the open, close, and afterhours prices of a stock on a specific date
///
/// # Arguments
///
/// * `client` - Reference to the Polygon client
/// * `ticker` - The ticker symbol
/// * `date` - The date (YYYY-MM-DD)
///
/// # Example
///
/// ```ignore
/// use polygon::{Polygon, rest::aggs};
///
/// let client = Polygon::new()?;
/// let data = aggs::daily_open_close(&client, "AAPL", "2023-01-09").await?;
/// ```
pub fn daily_open_close<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: &str,
    date: &str,
) -> Query<'a, Client> {
    Query::new(client, format!("https://api.polygon.io/v1/open-close/{}/{}", ticker, date))
        .optional("adjusted")
}
