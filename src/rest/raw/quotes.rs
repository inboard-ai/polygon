//! Quotes endpoint implementations

use crate::client::Polygon;
use crate::error::{Error, Result};
use crate::request::{Request, Response};

/// Get the last quote for a stock
///
/// # Arguments
///
/// * `client` - Reference to the Polygon client
/// * `ticker` - The stock ticker symbol
///
/// # Example
///
/// ```ignore
/// use polygon::{Polygon, rest::quotes};
///
/// let client = Polygon::new()?;
/// let quote = quotes::get_last_quote(&client, "AAPL").await?;
/// ```
pub async fn get_last_quote<Client: Request>(
    client: &Polygon<Client>,
    ticker: &str,
) -> Result<String> {
    let api_key = client.api_key().ok_or(Error::MissingApiKey)?;
    let url = format!(
        "https://api.polygon.io/v2/last/nbbo/{}?apiKey={}",
        ticker, api_key
    );
    let response = client.client().get(&url).await?;
    Ok(response.body())
}

/// Get the last forex quote
///
/// # Arguments
///
/// * `client` - Reference to the Polygon client
/// * `from` - The "from" symbol of the pair
/// * `to` - The "to" symbol of the pair
///
/// # Example
///
/// ```ignore
/// use polygon::{Polygon, rest::quotes};
///
/// let client = Polygon::new()?;
/// let quote = quotes::get_last_forex_quote(&client, "USD", "EUR").await?;
/// ```
pub async fn get_last_forex_quote<Client: Request>(
    client: &Polygon<Client>,
    from: &str,
    to: &str,
) -> Result<String> {
    let api_key = client.api_key().ok_or(Error::MissingApiKey)?;
    let url = format!(
        "https://api.polygon.io/v1/last_quote/currencies/{}/{}?apiKey={}",
        from, to, api_key
    );
    let response = client.client().get(&url).await?;
    Ok(response.body())
}
