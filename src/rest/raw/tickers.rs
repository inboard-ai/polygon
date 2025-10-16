//! Ticker endpoint implementations

use crate::client::Polygon;
use crate::query::Query;
use crate::request::Request;

/// Get a list of all tickers
pub fn all_tickers<'a, Client: Request>(client: &'a Polygon<Client>) -> Query<'a, Client> {
    Query::new(client, "https://api.polygon.io/v3/reference/tickers")
}

/// Get detailed information about a ticker
pub fn ticker_details<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: &str,
) -> Query<'a, Client> {
    Query::new(
        client,
        format!("https://api.polygon.io/v3/reference/tickers/{}", ticker),
    )
}

/// Get tickers related to a given ticker
pub fn related_tickers<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: &str,
) -> Query<'a, Client> {
    Query::new(
        client,
        format!("https://api.polygon.io/v1/related-companies/{}", ticker),
    )
}

/// Get all ticker types
pub fn ticker_types<'a, Client: Request>(client: &'a Polygon<Client>) -> Query<'a, Client> {
    Query::new(client, "https://api.polygon.io/v3/reference/tickers/types")
}
