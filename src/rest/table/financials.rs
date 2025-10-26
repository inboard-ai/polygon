//! Financials endpoints returning Polars DataFrames
use crate::client::Polygon;
use crate::processor::Table;
use crate::request::Request;
use crate::request::financials::Financials;

/// Get balance sheet data for public companies
///
/// Returns a request builder that will return results as a Polars DataFrame.
/// Use builder methods like `.ticker()`, `.cik()`, `.filing_date()`, `.limit()` to customize the request.
///
/// # Example
/// ```no_run
/// # use polygon::Client;
/// # use polygon::execute::Execute;
/// # async fn example() {
/// # let client = Client::new("api-key");
/// let df = polygon::rest::table::financials::balance_sheets(&client)
///     .ticker("AAPL")
///     .limit(10)
///     .get()
///     .await
///     .unwrap();
/// # }
/// ```
pub fn balance_sheets<'a, Client: Request>(client: &'a Polygon<Client>) -> Financials<'a, Client, Table> {
    Financials::balance_sheets(client).as_dataframe()
}

/// Get cash flow statement data for public companies
///
/// Returns a request builder that will return results as a Polars DataFrame.
/// Use builder methods like `.ticker()`, `.cik()`, `.filing_date()`, `.limit()` to customize the request.
pub fn cash_flow_statements<'a, Client: Request>(client: &'a Polygon<Client>) -> Financials<'a, Client, Table> {
    Financials::cash_flow_statements(client).as_dataframe()
}

/// Get income statement data for public companies
///
/// Returns a request builder that will return results as a Polars DataFrame.
/// Use builder methods like `.ticker()`, `.cik()`, `.filing_date()`, `.limit()` to customize the request.
pub fn income_statements<'a, Client: Request>(client: &'a Polygon<Client>) -> Financials<'a, Client, Table> {
    Financials::income_statements(client).as_dataframe()
}

/// Get financial ratios data for public companies
///
/// Returns a request builder that will return results as a Polars DataFrame.
/// Use builder methods like `.ticker()`, `.cik()`, `.filing_date()`, `.limit()` to customize the request.
pub fn ratios<'a, Client: Request>(client: &'a Polygon<Client>) -> Financials<'a, Client, Table> {
    Financials::ratios(client).as_dataframe()
}
