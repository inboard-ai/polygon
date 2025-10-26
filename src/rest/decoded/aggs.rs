//! Decoded aggregate endpoints - returns typed data instead of JSON strings

use crate::client::Polygon;
use crate::processor::Decoder;
use crate::request::Request;
use crate::request::aggs::{Aggregates, DailyOpenClose, GroupedDaily, PreviousClose};
use crate::request::common::Timespan;
use crate::response::aggs::{Agg, DailyOpenCloseAgg, GroupedDailyAgg, PreviousCloseAgg};
use crate::rest::raw;

/// Get aggregate bars for a stock over a given date range
///
/// Returns a decoded request builder that will return typed `Vec<`[`Agg`]`>` data.
/// Use builder methods like `.adjusted()`, `.sort()`, `.limit()` to customize the request.
pub fn aggregates<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: impl Into<String>,
    multiplier: u32,
    timespan: Timespan,
    from: impl Into<String>,
    to: impl Into<String>,
) -> Aggregates<'a, Client, Decoder<Vec<Agg>>> {
    raw::aggs::aggregates(client, ticker, multiplier, timespan, from, to).decoded()
}

/// Get the previous day's OHLC for a stock
///
/// Returns a decoded request builder that will return typed `Vec<`[`PreviousCloseAgg`]`>` data.
/// Use builder methods like `.adjusted()` to customize the request.
pub fn previous_close<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: impl Into<String>,
) -> PreviousClose<'a, Client, Decoder<Vec<PreviousCloseAgg>>> {
    raw::aggs::previous_close(client, ticker).decoded()
}

/// Get daily OHLC for the entire market
///
/// Returns a decoded request builder that will return typed `Vec<`[`GroupedDailyAgg`]`>` data.
/// Use builder methods like `.adjusted()` and `.include_otc()` to customize the request.
pub fn grouped_daily<'a, Client: Request>(
    client: &'a Polygon<Client>,
    date: impl Into<String>,
) -> GroupedDaily<'a, Client, Decoder<Vec<GroupedDailyAgg>>> {
    raw::aggs::grouped_daily(client, date).decoded()
}

/// Get the open/close/afterhours prices of a stock on a specific date
///
/// Returns a decoded request builder that will return typed DailyOpenCloseAgg data.
/// Use builder methods like `.adjusted()` to customize the request.
pub fn daily_open_close<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: impl Into<String>,
    date: impl Into<String>,
) -> DailyOpenClose<'a, Client, Decoder<DailyOpenCloseAgg>> {
    raw::aggs::daily_open_close(client, ticker, date).decoded()
}
