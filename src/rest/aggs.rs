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
pub async fn aggregates<Client: Request>(
    client: &Polygon<Client>,
    ticker: &str,
    multiplier: u32,
    timespan: &str,
    from: &str,
    to: &str,
    adjusted: Option<bool>,
    sort: Option<&str>,
    limit: Option<u32>,
) -> Result<String> {
    let api_key = client.api_key().ok_or(Error::MissingApiKey)?;
    let adjusted = adjusted.unwrap_or(true);
    let sort = sort.unwrap_or("asc");

    let mut url = format!(
        "https://api.polygon.io/v2/aggs/ticker/{}/range/{}/{}/{}/{}?adjusted={}&sort={}&apiKey={}",
        ticker, multiplier, timespan, from, to, adjusted, sort, api_key
    );

    if let Some(limit) = limit {
        url.push_str(&format!("&limit={}", limit));
    }

    client.client().get(&url).await
}

/// Get the previous day's open, high, low, and close (OHLC) for a stock
///
/// # Arguments
///
/// * `client` - Reference to the Polygon client
/// * `ticker` - The ticker symbol
/// * `adjusted` - Whether results are adjusted for splits (default: true)
///
/// # Example
///
/// ```ignore
/// use polygon::{Polygon, rest::aggs};
///
/// let client = Polygon::new()?;
/// let data = aggs::previous_close(&client, "AAPL", None).await?;
/// ```
pub async fn previous_close<Client: Request>(
    client: &Polygon<Client>,
    ticker: &str,
    adjusted: Option<bool>,
) -> Result<String> {
    let api_key = client.api_key().ok_or(Error::MissingApiKey)?;
    let adjusted = adjusted.unwrap_or(true);
    let url = format!(
        "https://api.polygon.io/v2/aggs/ticker/{}/prev?adjusted={}&apiKey={}",
        ticker, adjusted, api_key
    );
    client.client().get(&url).await
}

/// Get the daily open, high, low, and close (OHLC) for the entire market
///
/// # Arguments
///
/// * `client` - Reference to the Polygon client
/// * `date` - The date for the aggregate window (YYYY-MM-DD)
/// * `adjusted` - Whether results are adjusted for splits (default: true)
/// * `locale` - The locale (default: "us")
/// * `market_type` - The market type (default: "stocks")
/// * `include_otc` - Include OTC securities (default: false)
///
/// # Example
///
/// ```ignore
/// use polygon::{Polygon, rest::aggs};
///
/// let client = Polygon::new()?;
/// let data = aggs::grouped_daily(&client, "2023-01-09", None, None, None, None).await?;
/// ```
pub async fn grouped_daily<Client: Request>(
    client: &Polygon<Client>,
    date: &str,
    adjusted: Option<bool>,
    locale: Option<&str>,
    market_type: Option<&str>,
    include_otc: Option<bool>,
) -> Result<String> {
    let api_key = client.api_key().ok_or(Error::MissingApiKey)?;
    let adjusted = adjusted.unwrap_or(true);
    let locale = locale.unwrap_or("us");
    let market_type = market_type.unwrap_or("stocks");
    let include_otc = include_otc.unwrap_or(false);

    let url = format!(
        "https://api.polygon.io/v2/aggs/grouped/locale/{}/market/{}/{}?adjusted={}&include_otc={}&apiKey={}",
        locale, market_type, date, adjusted, include_otc, api_key
    );
    client.client().get(&url).await
}

/// Get the open, close, and afterhours prices of a stock on a specific date
///
/// # Arguments
///
/// * `client` - Reference to the Polygon client
/// * `ticker` - The ticker symbol
/// * `date` - The date (YYYY-MM-DD)
/// * `adjusted` - Whether results are adjusted for splits (default: true)
///
/// # Example
///
/// ```ignore
/// use polygon::{Polygon, rest::aggs};
///
/// let client = Polygon::new()?;
/// let data = aggs::daily_open_close(&client, "AAPL", "2023-01-09", None).await?;
/// ```
pub async fn daily_open_close<Client: Request>(
    client: &Polygon<Client>,
    ticker: &str,
    date: &str,
    adjusted: Option<bool>,
) -> Result<String> {
    let api_key = client.api_key().ok_or(Error::MissingApiKey)?;
    let adjusted = adjusted.unwrap_or(true);
    let url = format!(
        "https://api.polygon.io/v1/open-close/{}/{}?adjusted={}&apiKey={}",
        ticker, date, adjusted, api_key
    );
    client.client().get(&url).await
}
