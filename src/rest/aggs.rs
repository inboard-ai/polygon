//! Aggregates (bars) endpoint implementations

use crate::client::Polygon;
use crate::error::{Error, Result};
use crate::request::Request;

/// Get aggregate bars for a stock over a given date range
///
/// # Arguments
///
/// * `client` - Reference to the Polygon client
/// * `ticker` - The ticker symbol
/// * `multiplier` - The size of the timespan multiplier (e.g., 1)
/// * `timespan` - The size of the time window (e.g., "minute", "hour", "day", "week", "month", "quarter", "year")
/// * `from` - The start of the aggregate time window (YYYY-MM-DD)
/// * `to` - The end of the aggregate time window (YYYY-MM-DD)
///
/// # Example
///
/// ```ignore
/// use polygon::{Polygon, rest::aggs};
///
/// let client = Polygon::new()?;
/// let data = aggs::get_aggregates(&client, "AAPL", 1, "day", "2023-01-01", "2023-01-31").await?;
/// ```
pub async fn get_aggregates<Client: Request>(
    client: &Polygon<Client>,
    ticker: &str,
    multiplier: u32,
    timespan: &str,
    from: &str,
    to: &str,
) -> Result<String> {
    let api_key = client.api_key().ok_or(Error::MissingApiKey)?;
    let url = format!(
        "https://api.polygon.io/v2/aggs/ticker/{}/range/{}/{}/{}/{}?adjusted=true&sort=asc&apiKey={}",
        ticker, multiplier, timespan, from, to, api_key
    );
    client.client().get(&url).await
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
/// let data = aggs::get_previous_close(&client, "AAPL").await?;
/// ```
pub async fn get_previous_close<Client: Request>(
    client: &Polygon<Client>,
    ticker: &str,
) -> Result<String> {
    let api_key = client.api_key().ok_or(Error::MissingApiKey)?;
    let url = format!(
        "https://api.polygon.io/v2/aggs/ticker/{}/prev?adjusted=true&apiKey={}",
        ticker, api_key
    );
    client.client().get(&url).await
}
