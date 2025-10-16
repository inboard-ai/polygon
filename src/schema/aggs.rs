//! Aggregate data types

/// Aggregate data for a ticker over a date range
#[derive(Debug, Clone)]
pub struct Agg {
    /// Opening price
    pub open: Option<f64>,
    /// High price
    pub high: Option<f64>,
    /// Low price
    pub low: Option<f64>,
    /// Closing price
    pub close: Option<f64>,
    /// Volume
    pub volume: Option<f64>,
    /// Volume weighted average price
    pub vwap: Option<f64>,
    /// Unix timestamp (milliseconds)
    pub timestamp: Option<i64>,
    /// Number of transactions
    pub transactions: Option<i64>,
    /// Whether this is OTC
    pub otc: Option<bool>,
}

/// Grouped daily aggregate data
#[derive(Debug, Clone)]
pub struct GroupedDailyAgg {
    /// Ticker symbol
    pub ticker: Option<String>,
    /// Opening price
    pub open: Option<f64>,
    /// High price
    pub high: Option<f64>,
    /// Low price
    pub low: Option<f64>,
    /// Closing price
    pub close: Option<f64>,
    /// Volume
    pub volume: Option<f64>,
    /// Volume weighted average price
    pub vwap: Option<f64>,
    /// Unix timestamp (milliseconds)
    pub timestamp: Option<i64>,
    /// Number of transactions
    pub transactions: Option<i64>,
    /// Whether this is OTC
    pub otc: Option<bool>,
}

/// Daily open/close aggregate data
#[derive(Debug, Clone)]
pub struct DailyOpenCloseAgg {
    /// After hours price
    pub after_hours: Option<f64>,
    /// Closing price
    pub close: Option<f64>,
    /// Date (YYYY-MM-DD)
    pub from: Option<String>,
    /// High price
    pub high: Option<f64>,
    /// Low price
    pub low: Option<f64>,
    /// Opening price
    pub open: Option<f64>,
    /// Pre-market price
    pub pre_market: Option<f64>,
    /// Status
    pub status: Option<String>,
    /// Ticker symbol
    pub symbol: Option<String>,
    /// Volume
    pub volume: Option<f64>,
    /// Whether this is OTC
    pub otc: Option<bool>,
}

/// Previous close aggregate data
#[derive(Debug, Clone)]
pub struct PreviousCloseAgg {
    /// Ticker symbol
    pub ticker: Option<String>,
    /// Closing price
    pub close: Option<f64>,
    /// High price
    pub high: Option<f64>,
    /// Low price
    pub low: Option<f64>,
    /// Opening price
    pub open: Option<f64>,
    /// Unix timestamp (milliseconds)
    pub timestamp: Option<i64>,
    /// Volume
    pub volume: Option<f64>,
    /// Volume weighted average price
    pub vwap: Option<f64>,
}

#[cfg(feature = "decoder")]
impl crate::query::Decodable for Agg {}

#[cfg(feature = "decoder")]
impl crate::query::Decodable for Vec<Agg> {}

#[cfg(feature = "decoder")]
impl crate::query::Decodable for GroupedDailyAgg {}

#[cfg(feature = "decoder")]
impl crate::query::Decodable for Vec<GroupedDailyAgg> {}

#[cfg(feature = "decoder")]
impl crate::query::Decodable for DailyOpenCloseAgg {}

#[cfg(feature = "decoder")]
impl crate::query::Decodable for PreviousCloseAgg {}

#[cfg(feature = "decoder")]
impl crate::query::Decodable for Vec<PreviousCloseAgg> {}
