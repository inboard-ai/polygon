//! Financials endpoint implementations returning raw JSON strings

use crate::client::Polygon;
use crate::processor::Raw;
use crate::request::Request;
use crate::request::financials::Financials;

/// Get balance sheet data for public companies
///
/// Returns a request builder that will return results as raw JSON string.
/// Use builder methods like `.ticker()`, `.cik()`, `.filing_date()`, `.limit()` to customize the request.
pub fn balance_sheets<'a, Client: Request>(client: &'a Polygon<Client>) -> Financials<'a, Client, Raw> {
    Financials::balance_sheets(client)
}

/// Get cash flow statement data for public companies
///
/// Returns a request builder that will return results as raw JSON string.
/// Use builder methods like `.ticker()`, `.cik()`, `.filing_date()`, `.limit()` to customize the request.
pub fn cash_flow_statements<'a, Client: Request>(client: &'a Polygon<Client>) -> Financials<'a, Client, Raw> {
    Financials::cash_flow_statements(client)
}

/// Get income statement data for public companies
///
/// Returns a request builder that will return results as raw JSON string.
/// Use builder methods like `.ticker()`, `.cik()`, `.filing_date()`, `.limit()` to customize the request.
pub fn income_statements<'a, Client: Request>(client: &'a Polygon<Client>) -> Financials<'a, Client, Raw> {
    Financials::income_statements(client)
}

/// Get financial ratios data for public companies
///
/// Returns a request builder that will return results as raw JSON string.
/// Use builder methods like `.ticker()`, `.cik()`, `.filing_date()`, `.limit()` to customize the request.
pub fn ratios<'a, Client: Request>(client: &'a Polygon<Client>) -> Financials<'a, Client, Raw> {
    Financials::ratios(client)
}

#[cfg(all(test, feature = "dotenvy"))]
mod tests {
    use super::*;

    fn setup() -> Polygon<reqwest::Client> {
        Polygon::new().expect("Failed to create client. Make sure POLYGON_API_KEY is set in .env file")
    }

    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored --test-threads=1
    async fn test_balance_sheets() {
        let client = setup();
        let result = balance_sheets(&client)
            .cik("0000320193") // Apple Inc.
            .limit("1")
            .get()
            .await;
        assert!(result.is_ok(), "Failed to fetch balance sheets: {result:?}");
    }

    #[tokio::test]
    #[ignore]
    async fn test_cash_flow_statements() {
        let client = setup();
        let result = cash_flow_statements(&client).cik("0000320193").limit("1").get().await;
        assert!(result.is_ok(), "Failed to fetch cash flow statements: {result:?}");
    }

    #[tokio::test]
    #[ignore]
    async fn test_income_statements() {
        let client = setup();
        let result = income_statements(&client).cik("0000320193").limit("1").get().await;
        assert!(result.is_ok(), "Failed to fetch income statements: {result:?}");
    }

    #[tokio::test]
    #[ignore]
    async fn test_ratios() {
        let client = setup();
        let result = ratios(&client).ticker("AAPL").limit("1").get().await;
        assert!(result.is_ok(), "Failed to fetch ratios: {result:?}");
    }
}
