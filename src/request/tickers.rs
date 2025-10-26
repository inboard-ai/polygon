//! Ticker and reference data request parameters
//!
//! This module provides request builders for retrieving ticker symbols, company details, and
//! related reference data from the Polygon.io API. These endpoints help with asset discovery,
//! company research, and building ticker-based applications.
//!
//! # Endpoints
//!
//! ## All Tickers
//! Retrieve a comprehensive list of ticker symbols supported by Polygon.io across various asset
//! classes (e.g., stocks, indices, forex, crypto). Each ticker entry provides essential details
//! such as symbol, name, market, currency, and active status. This endpoint enables filtering by
//! ticker type, market, exchange, and other criteria to narrow down results.
//!
//! **Use Cases:** Asset discovery, data integration, filtering/selection, and application development.
//!
//! ## Ticker Overview (Details)
//! Retrieve comprehensive details for a single ticker supported by Polygon.io. This endpoint offers
//! a deep look into a company's fundamental attributes, including its primary exchange, standardized
//! identifiers (CIK, composite FIGI, share class FIGI), market capitalization, industry classification,
//! and key dates. Users also gain access to branding assets (e.g., logos, icons), enabling them to
//! enrich applications and analyses with visually consistent, contextually relevant information.
//!
//! **Use Cases:** Company research, data integration, application enhancement, due diligence & compliance.
//!
//! ## Related Tickers
//! Retrieve a list of tickers related to a specified ticker, identified through an analysis of news
//! coverage and returns data. This endpoint helps users discover peers, competitors, or thematically
//! similar companies, aiding in comparative analysis, portfolio diversification, and market research.
//!
//! **Use Cases:** Peer identification, comparative analysis, portfolio diversification, market research.
//!
//! ## Ticker Events
//! Retrieve corporate events data for a ticker, including stock splits, dividends, and other
//! significant corporate actions that may affect stock prices and trading strategies.
//!
//! **Use Cases:** Event tracking, corporate action analysis, trading strategy development.
//!
//! ## News
//! Retrieve news articles related to specific tickers or general market news. This endpoint provides
//! access to real-time and historical news coverage to support sentiment analysis and informed
//! decision-making.
//!
//! **Use Cases:** Sentiment analysis, news monitoring, market research, event-driven trading.

/// All tickers request builder implementation
pub mod all;
/// Ticker details (overview) request builder implementation
pub mod details;
/// Ticker events request builder implementation
pub mod events;
/// News articles request builder implementation
pub mod news;
/// Related tickers request builder implementation
pub mod related;
/// Ticker types request builder implementation
pub mod types;

pub use all::All;
pub use details::Details;
pub use events::Events;
pub use news::News;
pub use related::Related;
pub use types::Types;

// Re-export raw endpoints for convenience
pub use crate::rest::raw::tickers::{all, details, events, news, related, types};
