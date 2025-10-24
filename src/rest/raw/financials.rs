//! Financials endpoint implementations

use crate::client::Polygon;
use crate::query::Query;
use crate::request::Request;

/// Get balance sheet data for public companies
///
/// # Arguments
///
/// * `client` - Reference to the Polygon client
///
/// # Example
///
/// ```no_run
/// use polygon::Polygon;
/// use polygon::rest::raw::financials;
/// use polygon::query::Execute as _;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Polygon::default().with_key("your_api_key");
/// let data = financials::balance_sheets(&client)
///     .with("cik", "0000320193")
///     .with("limit", "10")
///     .get().await?;
/// # Ok(())
/// # }
/// ```
pub fn balance_sheets<'a, Client: Request>(client: &'a Polygon<Client>) -> Query<'a, Client> {
    Query::new(
        client,
        "https://api.polygon.io/stocks/financials/v1/balance-sheets",
    )
    .optional("cik")
    .optional("cik.any_of")
    .optional("cik.gt")
    .optional("cik.gte")
    .optional("cik.lt")
    .optional("cik.lte")
    .optional("tickers")
    .optional("tickers.all_of")
    .optional("tickers.any_of")
    .optional("period_end")
    .optional("period_end.gt")
    .optional("period_end.gte")
    .optional("period_end.lt")
    .optional("period_end.lte")
    .optional("filing_date")
    .optional("filing_date.gt")
    .optional("filing_date.gte")
    .optional("filing_date.lt")
    .optional("filing_date.lte")
    .optional("fiscal_year")
    .optional("fiscal_year.gt")
    .optional("fiscal_year.gte")
    .optional("fiscal_year.lt")
    .optional("fiscal_year.lte")
    .optional("fiscal_quarter")
    .optional("fiscal_quarter.gt")
    .optional("fiscal_quarter.gte")
    .optional("fiscal_quarter.lt")
    .optional("fiscal_quarter.lte")
    .optional("timeframe")
    .optional("timeframe.any_of")
    .optional("timeframe.gt")
    .optional("timeframe.gte")
    .optional("timeframe.lt")
    .optional("timeframe.lte")
    .optional("limit")
    .optional("sort")
}

/// Get cash flow statement data for public companies
///
/// # Arguments
///
/// * `client` - Reference to the Polygon client
///
/// # Example
///
/// ```no_run
/// use polygon::Polygon;
/// use polygon::rest::raw::financials;
/// use polygon::query::Execute as _;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Polygon::default().with_key("your_api_key");
/// let data = financials::cash_flow_statements(&client)
///     .with("cik", "0000320193")
///     .with("timeframe", "annual")
///     .get().await?;
/// # Ok(())
/// # }
/// ```
pub fn cash_flow_statements<'a, Client: Request>(client: &'a Polygon<Client>) -> Query<'a, Client> {
    Query::new(
        client,
        "https://api.polygon.io/stocks/financials/v1/cash-flow-statements",
    )
    .optional("cik")
    .optional("cik.any_of")
    .optional("cik.gt")
    .optional("cik.gte")
    .optional("cik.lt")
    .optional("cik.lte")
    .optional("tickers")
    .optional("tickers.all_of")
    .optional("tickers.any_of")
    .optional("period_end")
    .optional("period_end.gt")
    .optional("period_end.gte")
    .optional("period_end.lt")
    .optional("period_end.lte")
    .optional("filing_date")
    .optional("filing_date.gt")
    .optional("filing_date.gte")
    .optional("filing_date.lt")
    .optional("filing_date.lte")
    .optional("fiscal_year")
    .optional("fiscal_year.gt")
    .optional("fiscal_year.gte")
    .optional("fiscal_year.lt")
    .optional("fiscal_year.lte")
    .optional("fiscal_quarter")
    .optional("fiscal_quarter.gt")
    .optional("fiscal_quarter.gte")
    .optional("fiscal_quarter.lt")
    .optional("fiscal_quarter.lte")
    .optional("timeframe")
    .optional("timeframe.any_of")
    .optional("timeframe.gt")
    .optional("timeframe.gte")
    .optional("timeframe.lt")
    .optional("timeframe.lte")
    .optional("limit")
    .optional("sort")
}

/// Get income statement data for public companies
///
/// # Arguments
///
/// * `client` - Reference to the Polygon client
///
/// # Example
///
/// ```no_run
/// use polygon::Polygon;
/// use polygon::rest::raw::financials;
/// use polygon::query::Execute as _;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Polygon::default().with_key("your_api_key");
/// let data = financials::income_statements(&client)
///     .with("cik", "0000320193")
///     .with("fiscal_year", "2023")
///     .get().await?;
/// # Ok(())
/// # }
/// ```
pub fn income_statements<'a, Client: Request>(client: &'a Polygon<Client>) -> Query<'a, Client> {
    Query::new(
        client,
        "https://api.polygon.io/stocks/financials/v1/income-statements",
    )
    .optional("cik")
    .optional("cik.any_of")
    .optional("cik.gt")
    .optional("cik.gte")
    .optional("cik.lt")
    .optional("cik.lte")
    .optional("tickers")
    .optional("tickers.all_of")
    .optional("tickers.any_of")
    .optional("period_end")
    .optional("period_end.gt")
    .optional("period_end.gte")
    .optional("period_end.lt")
    .optional("period_end.lte")
    .optional("filing_date")
    .optional("filing_date.gt")
    .optional("filing_date.gte")
    .optional("filing_date.lt")
    .optional("filing_date.lte")
    .optional("fiscal_year")
    .optional("fiscal_year.gt")
    .optional("fiscal_year.gte")
    .optional("fiscal_year.lt")
    .optional("fiscal_year.lte")
    .optional("fiscal_quarter")
    .optional("fiscal_quarter.gt")
    .optional("fiscal_quarter.gte")
    .optional("fiscal_quarter.lt")
    .optional("fiscal_quarter.lte")
    .optional("timeframe")
    .optional("timeframe.any_of")
    .optional("timeframe.gt")
    .optional("timeframe.gte")
    .optional("timeframe.lt")
    .optional("timeframe.lte")
    .optional("limit")
    .optional("sort")
}

/// Get financial ratios data for public companies
///
/// # Arguments
///
/// * `client` - Reference to the Polygon client
///
/// # Example
///
/// ```no_run
/// use polygon::Polygon;
/// use polygon::rest::raw::financials;
/// use polygon::query::Execute as _;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Polygon::default().with_key("your_api_key");
/// let data = financials::ratios(&client)
///     .with("ticker", "AAPL")
///     .with("limit", "10")
///     .get().await?;
/// # Ok(())
/// # }
/// ```
pub fn ratios<'a, Client: Request>(client: &'a Polygon<Client>) -> Query<'a, Client> {
    Query::new(client, "https://api.polygon.io/stocks/financials/v1/ratios")
        .optional("ticker")
        .optional("ticker.any_of")
        .optional("ticker.gt")
        .optional("ticker.gte")
        .optional("ticker.lt")
        .optional("ticker.lte")
        .optional("cik")
        .optional("cik.any_of")
        .optional("cik.gt")
        .optional("cik.gte")
        .optional("cik.lt")
        .optional("cik.lte")
        .optional("price")
        .optional("price.gt")
        .optional("price.gte")
        .optional("price.lt")
        .optional("price.lte")
        .optional("average_volume")
        .optional("average_volume.gt")
        .optional("average_volume.gte")
        .optional("average_volume.lt")
        .optional("average_volume.lte")
        .optional("market_cap")
        .optional("market_cap.gt")
        .optional("market_cap.gte")
        .optional("market_cap.lt")
        .optional("market_cap.lte")
        .optional("earnings_per_share")
        .optional("earnings_per_share.gt")
        .optional("earnings_per_share.gte")
        .optional("earnings_per_share.lt")
        .optional("earnings_per_share.lte")
        .optional("price_to_earnings")
        .optional("price_to_earnings.gt")
        .optional("price_to_earnings.gte")
        .optional("price_to_earnings.lt")
        .optional("price_to_earnings.lte")
        .optional("price_to_book")
        .optional("price_to_book.gt")
        .optional("price_to_book.gte")
        .optional("price_to_book.lt")
        .optional("price_to_book.lte")
        .optional("price_to_sales")
        .optional("price_to_sales.gt")
        .optional("price_to_sales.gte")
        .optional("price_to_sales.lt")
        .optional("price_to_sales.lte")
        .optional("limit")
        .optional("sort")
}

#[cfg(all(test, feature = "dotenvy"))]
mod tests {
    use super::*;
    use crate::query::Execute as _;

    fn setup() -> Polygon<reqwest::Client> {
        Polygon::new()
            .expect("Failed to create client. Make sure POLYGON_API_KEY is set in .env file")
    }

    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored --test-threads=1
    async fn test_balance_sheets() {
        let client = setup();
        let result = balance_sheets(&client)
            .param("cik", "0000320193") // Apple Inc.
            .param("limit", "1")
            .get()
            .await;
        assert!(
            result.is_ok(),
            "Failed to fetch balance sheets: {:?}",
            result
        );
    }

    #[tokio::test]
    #[ignore]
    async fn test_cash_flow_statements() {
        let client = setup();
        let result = cash_flow_statements(&client)
            .param("cik", "0000320193")
            .param("limit", "1")
            .get()
            .await;
        assert!(
            result.is_ok(),
            "Failed to fetch cash flow statements: {:?}",
            result
        );
    }

    #[tokio::test]
    #[ignore]
    async fn test_income_statements() {
        let client = setup();
        let result = income_statements(&client)
            .param("cik", "0000320193")
            .param("limit", "1")
            .get()
            .await;
        assert!(
            result.is_ok(),
            "Failed to fetch income statements: {:?}",
            result
        );
    }

    #[tokio::test]
    #[ignore]
    async fn test_ratios() {
        let client = setup();
        let result = ratios(&client)
            .param("ticker", "AAPL")
            .param("limit", "1")
            .get()
            .await;
        assert!(result.is_ok(), "Failed to fetch ratios: {:?}", result);
    }
}
