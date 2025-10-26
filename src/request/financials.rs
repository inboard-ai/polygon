//! Financial data request parameters
//!
//! This module provides request builders for retrieving fundamental financial data for public
//! companies from the Polygon.io API. All endpoints return comprehensive financial statement data
//! including quarterly, annual, and trailing twelve-month (TTM) periods.
//!
//! # Endpoints
//!
//! ## Balance Sheets
//! Retrieve comprehensive balance sheet data for public companies, containing quarterly and annual
//! financial positions. This dataset includes detailed asset, liability, and equity positions
//! representing the company's financial position at specific points in time. Balance sheet data
//! represents point-in-time snapshots rather than cumulative flows, showing what the company owns,
//! owes, and shareholders' equity as of each period end date.
//!
//! **Use Cases:** Financial analysis, company valuation, asset assessment, debt analysis, equity research.
//!
//! ## Cash Flow Statements
//! Retrieve comprehensive cash flow statement data for public companies, including quarterly, annual,
//! and trailing twelve-month cash flows. This dataset includes detailed operating, investing, and
//! financing cash flows with TTM calculations that sum all cash flow components over four quarters.
//!
//! **Use Cases:** Cash flow analysis, liquidity assessment, operational efficiency evaluation, investment activity tracking.
//!
//! ## Income Statements
//! Retrieve comprehensive income statement data for public companies, including key metrics such as
//! revenue, expenses, and net income for various reporting periods. This dataset provides detailed
//! financial performance data including revenue breakdowns, operating expenses, and profitability
//! metrics across quarterly, annual, and trailing twelve-month periods.
//!
//! **Use Cases:** Profitability analysis, revenue trend analysis, expense management evaluation, earnings assessment.
//!
//! ## Financial Ratios
//! Retrieve comprehensive financial ratios data providing key valuation, profitability, liquidity,
//! and leverage metrics for public companies. This dataset combines data from income statements,
//! balance sheets, and cash flow statements with daily stock prices to calculate ratios for the
//! most recent trading day using trailing twelve months (TTM) financials.
//!
//! **Use Cases:** Company valuation, comparative analysis, financial health assessment, investment screening.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::client::Polygon;
use crate::error::Result;
use crate::execute::Execute;
use crate::processor::{Decoder, Processor, Raw};
use crate::request::Request;
use crate::request::common::SortOrder;

use super::common::Limit;

// Re-export raw endpoints for convenience
pub use crate::rest::raw::financials::{balance_sheets, cash_flow_statements, income_statements, ratios};

/// Request builder for financial data (balance sheets, cash flow statements, income statements, ratios)
pub struct Financials<'a, Client: Request, P: Processor = Raw> {
    client: &'a Polygon<Client>,
    endpoint_path: &'static str,
    /// Ticker symbol to filter by (e.g., "AAPL" for Apple Inc.)
    pub ticker: Option<String>,
    /// Central Index Key (CIK) assigned by the SEC to identify the company
    pub cik: Option<String>,
    /// Date when the financial statement was filed with the SEC (YYYY-MM-DD)
    pub filing_date: Option<String>,
    /// The last date of the reporting period (YYYY-MM-DD)
    pub period_of_report_date: Option<String>,
    /// Maximum number of results to return
    pub limit: Option<u32>,
    /// Sort order for results (asc or desc)
    pub order: Option<SortOrder>,
    processor: P,
}

impl<'a, C: Request> Financials<'a, C, Raw> {
    /// Create a new balance sheets request
    pub fn balance_sheets(client: &'a Polygon<C>) -> Self {
        Self::new(client, "/vX/reference/financials")
    }

    /// Create a new cash flow statements request
    pub fn cash_flow_statements(client: &'a Polygon<C>) -> Self {
        Self::new(client, "/vX/reference/financials")
    }

    /// Create a new income statements request
    pub fn income_statements(client: &'a Polygon<C>) -> Self {
        Self::new(client, "/vX/reference/financials")
    }

    /// Create a new financial ratios request
    pub fn ratios(client: &'a Polygon<C>) -> Self {
        Self::new(client, "/vX/reference/financials")
    }

    fn new(client: &'a Polygon<C>, endpoint_path: &'static str) -> Self {
        Self {
            client,
            endpoint_path,
            ticker: None,
            cik: None,
            filing_date: None,
            period_of_report_date: None,
            limit: None,
            order: None,
            processor: Raw,
        }
    }
}

impl<'a, C: Request, P: Processor + 'a> Financials<'a, C, P> {
    /// Execute the request and return the result
    pub fn get(self) -> impl std::future::Future<Output = Result<P::Output>> + 'a {
        Execute::get(self)
    }

    /// Convert to decoded typed output
    ///
    /// The return type T is inferred from context. For example:
    /// - `decoded::<Vec<BalanceSheet>>()` for balance sheets
    /// - `decoded::<Vec<CashFlowStatement>>()` for cash flow statements
    /// - `decoded::<Vec<IncomeStatement>>()` for income statements  
    /// - `decoded::<Vec<FinancialRatio>>()` for ratios
    pub fn decoded<T>(self) -> Financials<'a, C, Decoder<T>>
    where
        T: crate::response::financials::DecodeFinancials,
    {
        let decoder = Decoder::new(T::decoder_fn());

        Financials {
            client: self.client,
            endpoint_path: self.endpoint_path,
            ticker: self.ticker,
            cik: self.cik,
            filing_date: self.filing_date,
            period_of_report_date: self.period_of_report_date,
            limit: self.limit,
            order: self.order,
            processor: decoder,
        }
    }

    /// Convert this request to return results as a Polars DataFrame instead of raw JSON
    #[cfg(feature = "table")]
    pub fn as_dataframe(self) -> Financials<'a, C, crate::processor::Table> {
        Financials {
            client: self.client,
            endpoint_path: self.endpoint_path,
            ticker: self.ticker,
            cik: self.cik,
            filing_date: self.filing_date,
            period_of_report_date: self.period_of_report_date,
            limit: self.limit,
            order: self.order,
            processor: crate::processor::Table,
        }
    }

    /// Set a custom decoder function to return typed data instead of raw JSON
    ///
    /// Since Financials is a generic builder for multiple endpoint types (balance sheets, cash flow, etc.),
    /// you need to provide a custom decoder function for the specific financial data type you want.
    pub fn with_decoder<T>(
        self,
        decoder_fn: impl Fn(decoder::Value) -> decoder::Result<T> + Send + Sync + 'static,
    ) -> Financials<'a, C, Decoder<T>> {
        Financials {
            client: self.client,
            endpoint_path: self.endpoint_path,
            ticker: self.ticker,
            cik: self.cik,
            filing_date: self.filing_date,
            period_of_report_date: self.period_of_report_date,
            limit: self.limit,
            order: self.order,
            processor: Decoder::new(decoder_fn),
        }
    }

    /// Filter by ticker symbol (e.g., "AAPL")
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    /// Filter by Central Index Key (CIK)
    pub fn cik(mut self, cik: impl Into<String>) -> Self {
        self.cik = Some(cik.into());
        self
    }

    /// Filter by filing date (YYYY-MM-DD)
    pub fn filing_date(mut self, date: impl Into<String>) -> Self {
        self.filing_date = Some(date.into());
        self
    }

    /// Filter by period of report date (YYYY-MM-DD)
    pub fn period_of_report_date(mut self, period: impl Into<String>) -> Self {
        self.period_of_report_date = Some(period.into());
        self
    }

    /// Set the maximum number of results to return
    pub fn limit(mut self, limit: impl Into<Limit>) -> Self {
        self.limit = limit.into().into();
        self
    }

    /// Set the sort order (asc or desc)
    pub fn order(mut self, order: SortOrder) -> Self {
        self.order = Some(order);
        self
    }
}

impl<'a, C: Request, P: Processor + 'a> Execute for Financials<'a, C, P> {
    type Output = P::Output;

    #[allow(refining_impl_trait_reachable)]
    async fn get(self) -> Result<P::Output> {
        let api_key = self
            .client
            .api_key()
            .ok_or_else(|| crate::error::Error::Custom("API key not set".to_string()))?;

        let mut path = self.endpoint_path.to_string();
        let mut params = vec![format!("apiKey={}", api_key)];

        if let Some(t) = self.ticker {
            params.push(format!("ticker={t}"));
        }
        if let Some(c) = self.cik {
            params.push(format!("cik={c}"));
        }
        if let Some(f) = self.filing_date {
            params.push(format!("filing_date={f}"));
        }
        if let Some(pr) = self.period_of_report_date {
            params.push(format!("period_of_report_date={pr}"));
        }
        if let Some(l) = self.limit {
            params.push(format!("limit={l}"));
        }
        if let Some(o) = self.order {
            params.push(format!("order={}", format!("{o:?}").to_lowercase()));
        }

        path.push('?');
        path.push_str(&params.join("&"));
        let url = format!("https://api.polygon.io{path}");
        let response = self.client.client().get(&url).await;
        self.processor.process(response)
    }
}

/// JSON-serializable parameters for financial data requests
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct Params {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Ticker symbol to filter by (e.g., "AAPL" for Apple Inc.)
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Central Index Key (CIK) assigned by the SEC to identify the company
    pub cik: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date when the financial statement was filed with the SEC (YYYY-MM-DD)
    pub filing_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The last date of the reporting period (YYYY-MM-DD)
    pub period_of_report_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum number of results to return
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sort order for results (asc or desc)
    pub order: Option<SortOrder>,
}
