//! Ticker endpoints returning Polars DataFrames
use crate::client::Polygon;
use crate::processor::Table;
use crate::request::Request;
use crate::request::tickers::{All, Details, Events, News, Related, Types};

/// Get a list of all tickers
///
/// Returns a request builder that will return results as a Polars DataFrame.
/// Use builder methods like `.ticker()`, `.market()`, `.exchange()`, `.limit()` to customize the request.
///
/// # Example
/// ```no_run
/// # use polygon::Client;
/// # use polygon::execute::Execute;
/// # async fn example() {
/// # let client = Client::new("api-key");
/// let df = polygon::rest::table::tickers::all(&client)
///     .market("stocks")
///     .limit(100)
///     .get()
///     .await
///     .unwrap();
/// # }
/// ```
pub fn all<'a, Client: Request>(client: &'a Polygon<Client>) -> All<'a, Client, Table> {
    All::new(client).as_dataframe()
}

/// Get detailed information about a ticker
///
/// Returns a request builder that will return results as a Polars DataFrame.
/// Use builder methods like `.date()` to customize the request.
///
/// # Example
/// ```no_run
/// # use polygon::Client;
/// # use polygon::execute::Execute;
/// # async fn example() {
/// # let client = Client::new("api-key");
/// let df = polygon::rest::table::tickers::details(&client, "AAPL")
///     .get()
///     .await
///     .unwrap();
/// # }
/// ```
pub fn details<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: impl Into<String>,
) -> Details<'a, Client, Table> {
    Details::new(client, ticker).as_dataframe()
}

/// Get tickers related to a given ticker
///
/// Returns a request builder that will return results as a Polars DataFrame.
///
/// # Example
/// ```no_run
/// # use polygon::Client;
/// # use polygon::execute::Execute;
/// # async fn example() {
/// # let client = Client::new("api-key");
/// let df = polygon::rest::table::tickers::related(&client, "AAPL")
///     .get()
///     .await
///     .unwrap();
/// # }
/// ```
pub fn related<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: impl Into<String>,
) -> Related<'a, Client, Table> {
    Related::new(client, ticker).as_dataframe()
}

/// Get all ticker types
///
/// Returns a request builder that will return results as a Polars DataFrame.
/// Use builder methods like `.asset_class()` and `.locale()` to customize the request.
///
/// # Example
/// ```no_run
/// # use polygon::Client;
/// # use polygon::execute::Execute;
/// # async fn example() {
/// # let client = Client::new("api-key");
/// let df = polygon::rest::table::tickers::types(&client)
///     .asset_class("stocks")
///     .get()
///     .await
///     .unwrap();
/// # }
/// ```
pub fn types<'a, Client: Request>(client: &'a Polygon<Client>) -> Types<'a, Client, Table> {
    Types::new(client).as_dataframe()
}

/// Get event history for a ticker
///
/// Returns a request builder that will return results as a Polars DataFrame.
/// Use builder methods like `.types()` to customize the request.
///
/// # Example
/// ```no_run
/// # use polygon::Client;
/// # use polygon::execute::Execute;
/// # async fn example() {
/// # let client = Client::new("api-key");
/// let df = polygon::rest::table::tickers::events(&client, "AAPL")
///     .get()
///     .await
///     .unwrap();
/// # }
/// ```
pub fn events<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: impl Into<String>,
) -> Events<'a, Client, Table> {
    Events::new(client, ticker).as_dataframe()
}

/// Get the most recent news articles
///
/// Returns a request builder that will return results as a Polars DataFrame.
/// Use builder methods like `.ticker()`, `.limit()`, `.order()` to customize the request.
///
/// # Example
/// ```no_run
/// # use polygon::Client;
/// # use polygon::execute::Execute;
/// # async fn example() {
/// # let client = Client::new("api-key");
/// let df = polygon::rest::table::tickers::news(&client)
///     .ticker("AAPL")
///     .limit(10)
///     .get()
///     .await
///     .unwrap();
/// # }
/// ```
pub fn news<'a, Client: Request>(client: &'a Polygon<Client>) -> News<'a, Client, Table> {
    News::new(client).as_dataframe()
}
