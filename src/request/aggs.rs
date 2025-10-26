//! Aggregate (OHLCV) data request parameters
//!
//! This module provides request builders for retrieving historical OHLC (Open, High, Low, Close)
//! and volume data for stocks from the Polygon.io API.
//!
//! # Endpoints
//!
//! ## Custom Bars (Aggregates)
//! Retrieve aggregated historical OHLC and volume data for a specified stock ticker over a custom
//! date range and time interval in Eastern Time (ET). Aggregates are constructed exclusively from
//! qualifying trades that meet specific conditions. If no eligible trades occur within a given
//! timeframe, no aggregate bar is produced, resulting in an empty interval that indicates a lack
//! of trading activity during that period.
//!
//! Users can tailor their data by adjusting the multiplier and timespan parameters (e.g., a 5-minute bar),
//! covering pre-market, regular market, and after-hours sessions. This flexibility supports a broad
//! range of analytical and visualization needs.
//!
//! **Use Cases:** Data visualization, technical analysis, backtesting strategies, market research.
//!
//! ## Previous Day Bar
//! Retrieve the previous trading day's open, high, low, and close (OHLC) data for a specified stock
//! ticker. This endpoint provides key pricing metrics, including volume, to help users assess recent
//! performance and inform trading strategies.
//!
//! **Use Cases:** Baseline comparison, technical analysis, market research, and daily reporting.
//!
//! ## Daily Market Summary (Grouped Daily)
//! Retrieve daily OHLC, volume, and volume-weighted average price (VWAP) data for all U.S. stocks
//! on a specified trading date. This endpoint returns comprehensive market coverage in a single request,
//! enabling wide-scale analysis, bulk data processing, and research into broad market performance.
//!
//! **Use Cases:** Market overview, bulk data processing, historical research, and portfolio comparison.
//!
//! ## Daily Open/Close
//! Retrieve the open, high, low, and close (OHLC) prices for a specific stock ticker on a given date,
//! including pre-market and after-hours data when available.
//!
//! **Use Cases:** Daily price analysis, historical data collection, and price tracking.

/// Aggregates (custom bars) request builder implementation
pub mod aggregates;
/// Daily open/close request builder implementation
pub mod daily_open_close;
/// Grouped daily bars request builder implementation
pub mod grouped_daily;
/// Previous close request builder implementation
pub mod previous_close;

pub use aggregates::Aggregates;
pub use daily_open_close::DailyOpenClose;
pub use grouped_daily::GroupedDaily;
pub use previous_close::PreviousClose;

// Re-export raw endpoints for convenience
pub use crate::rest::raw::aggs::{aggregates, daily_open_close, grouped_daily, previous_close};
