//! Type-safe endpoint definitions
//!
//! This module defines all API endpoints as nested enums where each variant
//! contains its request parameters. The type system enforces Request → Response correctness.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::request::aggs;
use crate::request::financials;
use crate::request::tickers;

/// All available Polygon API endpoints
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Endpoint {
    /// Ticker endpoints
    Tickers(Tickers),
    /// Aggregates/OHLCV endpoints  
    Aggs(Aggs),
    /// Financial data endpoints
    Financials(Financials),
}

/// Ticker-related endpoints
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "endpoint", content = "params")]
pub enum Tickers {
    /// List all tickers
    #[serde(rename = "all")]
    All(tickers::all::Params),

    /// Get detailed information about a specific ticker
    #[serde(rename = "details")]
    Details(tickers::details::Params),

    /// Get tickers related to a given ticker
    #[serde(rename = "related")]
    Related(tickers::related::Params),

    /// Get all ticker types
    #[serde(rename = "types")]
    Types,

    /// Get corporate events for a ticker
    #[serde(rename = "events")]
    Events(tickers::events::Params),

    /// Get recent news articles
    #[serde(rename = "news")]
    News(tickers::news::Params),
}

/// Aggregates/OHLCV endpoints
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "endpoint", content = "params")]
pub enum Aggs {
    /// Get aggregate bars for a stock over a date range
    #[serde(rename = "aggregates")]
    Aggregates(aggs::aggregates::Params),

    /// Get the previous day's OHLC for a stock
    #[serde(rename = "previous_close")]
    PreviousClose(aggs::previous_close::Params),

    /// Get daily OHLCV data for the entire stock market
    #[serde(rename = "grouped_daily")]
    GroupedDaily(aggs::grouped_daily::Params),

    /// Get open, close, and afterhours prices for a stock on a date
    #[serde(rename = "daily_open_close")]
    DailyOpenClose(aggs::daily_open_close::Params),
}

/// Financial data endpoints
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "endpoint", content = "params")]
pub enum Financials {
    /// Get balance sheet data
    #[serde(rename = "balance_sheets")]
    BalanceSheets(financials::Params),

    /// Get cash flow statement data
    #[serde(rename = "cash_flow_statements")]
    CashFlowStatements(financials::Params),

    /// Get income statement data
    #[serde(rename = "income_statements")]
    IncomeStatements(financials::Params),

    /// Get financial ratios
    #[serde(rename = "ratios")]
    Ratios(financials::Params),
}
