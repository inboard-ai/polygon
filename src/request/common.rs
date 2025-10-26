//! Common types used across multiple endpoints
use std::str::FromStr;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Sort order
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    /// Ascending
    Asc,
    /// Descending
    Desc,
    /// Custom
    #[serde(serialize_with = "serialize_custom")]
    Custom(String),
}

fn serialize_custom<S>(value: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(value)
}

impl From<&str> for SortOrder {
    fn from(value: &str) -> Self {
        match value {
            "asc" => SortOrder::Asc,
            "desc" => SortOrder::Desc,
            _ => SortOrder::Custom(value.to_string()),
        }
    }
}

impl From<String> for SortOrder {
    fn from(value: String) -> Self {
        SortOrder::from(value.as_str())
    }
}

impl From<SortOrder> for String {
    fn from(value: SortOrder) -> Self {
        match value {
            SortOrder::Asc => "asc".to_string(),
            SortOrder::Desc => "desc".to_string(),
            SortOrder::Custom(s) => s,
        }
    }
}

/// Timespan for aggregate data
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Timespan {
    /// Minute bars
    Minute,
    /// Hour bars
    Hour,
    /// Day bars
    Day,
    /// Week bars
    Week,
    /// Month bars
    Month,
    /// Quarter bars
    Quarter,
    /// Year bars
    Year,
}

impl FromStr for Timespan {
    type Err = crate::error::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "minute" | "min" => Ok(Timespan::Minute),
            "hour" | "hr" | "h" => Ok(Timespan::Hour),
            "day" | "d" | "dy" => Ok(Timespan::Day),
            "week" | "w" | "wk" => Ok(Timespan::Week),
            "month" | "mo" | "mth" => Ok(Timespan::Month),
            "quarter" | "q" | "qrtr" | "qtr" => Ok(Timespan::Quarter),
            "year" | "y" | "yr" => Ok(Timespan::Year),
            _ => Err(crate::error::Error::Custom(format!("Invalid timespan: {s}"))),
        }
    }
}

/// Limit for number of results
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub enum Limit {
    /// No limit
    None,
    /// Some limit
    Some(u32),
}

macro_rules! limit_from {
    ($t:ty) => {
        impl From<$t> for Limit {
            fn from(value: $t) -> Self {
                Limit::Some(value as u32)
            }
        }
    };
}

limit_from!(u32);
limit_from!(i32);
limit_from!(u64);
limit_from!(i64);
limit_from!(usize);
limit_from!(isize);

impl From<&str> for Limit {
    fn from(value: &str) -> Self {
        match value.parse() {
            Ok(v) => Limit::Some(v),
            Err(_) => Limit::None,
        }
    }
}

impl From<String> for Limit {
    fn from(value: String) -> Self {
        Limit::from(value.as_str())
    }
}

impl From<Limit> for Option<u32> {
    fn from(value: Limit) -> Self {
        match value {
            Limit::None => None,
            Limit::Some(v) => Some(v),
        }
    }
}
