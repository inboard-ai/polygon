//! Decoded aggregate endpoints - returns typed data instead of JSON strings

use crate::client::Polygon;
use crate::query::Query;
use crate::request::Request;
use crate::rest::raw;
use crate::schema::aggs::{Agg, DailyOpenCloseAgg, GroupedDailyAgg, PreviousCloseAgg};

/// Get aggregate bars for a stock over a given date range
pub fn aggregates<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: &str,
    multiplier: u32,
    timespan: &str,
    from: &str,
    to: &str,
) -> Query<'a, Client, Vec<Agg>> {
    raw::aggs::aggregates(client, ticker, multiplier, timespan, from, to)
        .with_decoder(|v| decode_results(v, decode_agg))
}

/// Get the previous day's OHLC for a stock
pub fn previous_close<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: &str,
) -> Query<'a, Client, Vec<PreviousCloseAgg>> {
    raw::aggs::previous_close(client, ticker)
        .with_decoder(|v| decode_results(v, decode_previous_close_agg))
}

/// Get daily OHLC for the entire market
pub fn grouped_daily<'a, Client: Request>(
    client: &'a Polygon<Client>,
    date: &str,
) -> Query<'a, Client, Vec<GroupedDailyAgg>> {
    raw::aggs::grouped_daily(client, date)
        .with_decoder(|v| decode_results(v, decode_grouped_daily_agg))
}

/// Get the open/close/afterhours prices of a stock on a specific date
pub fn daily_open_close<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: &str,
    date: &str,
) -> Query<'a, Client, DailyOpenCloseAgg> {
    raw::aggs::daily_open_close(client, ticker, date).with_decoder(decode_daily_open_close)
}

/// Generic helper to decode API responses with a "results" array
fn decode_results<T>(
    value: decoder::Value,
    item_decoder: impl Fn(decoder::Value) -> decoder::Result<T>,
) -> decoder::Result<Vec<T>> {
    let mut response = decoder::decode::map(value)?;
    response.required("results", decoder::decode::sequence(item_decoder))
}

fn decode_agg(value: decoder::Value) -> decoder::Result<Agg> {
    let mut agg = decoder::decode::map(value)?;

    Ok(Agg {
        open: agg.optional("o", decoder::decode::f64)?,
        high: agg.optional("h", decoder::decode::f64)?,
        low: agg.optional("l", decoder::decode::f64)?,
        close: agg.optional("c", decoder::decode::f64)?,
        volume: agg.optional("v", decoder::decode::f64)?,
        vwap: agg.optional("vw", decoder::decode::f64)?,
        timestamp: agg.optional("t", decoder::decode::i64)?,
        transactions: agg.optional("n", decoder::decode::i64)?,
        otc: agg.optional("otc", decoder::decode::bool)?,
    })
}

fn decode_previous_close_agg(value: decoder::Value) -> decoder::Result<PreviousCloseAgg> {
    let mut prev = decoder::decode::map(value)?;

    Ok(PreviousCloseAgg {
        ticker: prev.optional("T", decoder::decode::string)?,
        close: prev.optional("c", decoder::decode::f64)?,
        high: prev.optional("h", decoder::decode::f64)?,
        low: prev.optional("l", decoder::decode::f64)?,
        open: prev.optional("o", decoder::decode::f64)?,
        timestamp: prev.optional("t", decoder::decode::i64)?,
        volume: prev.optional("v", decoder::decode::f64)?,
        vwap: prev.optional("vw", decoder::decode::f64)?,
    })
}

fn decode_grouped_daily_agg(value: decoder::Value) -> decoder::Result<GroupedDailyAgg> {
    let mut grouped = decoder::decode::map(value)?;

    Ok(GroupedDailyAgg {
        ticker: grouped.optional("T", decoder::decode::string)?,
        open: grouped.optional("o", decoder::decode::f64)?,
        high: grouped.optional("h", decoder::decode::f64)?,
        low: grouped.optional("l", decoder::decode::f64)?,
        close: grouped.optional("c", decoder::decode::f64)?,
        volume: grouped.optional("v", decoder::decode::f64)?,
        vwap: grouped.optional("vw", decoder::decode::f64)?,
        timestamp: grouped.optional("t", decoder::decode::i64)?,
        transactions: grouped.optional("n", decoder::decode::i64)?,
        otc: grouped.optional("otc", decoder::decode::bool)?,
    })
}

fn decode_daily_open_close(value: decoder::Value) -> decoder::Result<DailyOpenCloseAgg> {
    let mut daily = decoder::decode::map(value)?;

    Ok(DailyOpenCloseAgg {
        after_hours: daily.optional("afterHours", decoder::decode::f64)?,
        close: daily.optional("close", decoder::decode::f64)?,
        from: daily.optional("from", decoder::decode::string)?,
        high: daily.optional("high", decoder::decode::f64)?,
        low: daily.optional("low", decoder::decode::f64)?,
        open: daily.optional("open", decoder::decode::f64)?,
        pre_market: daily.optional("preMarket", decoder::decode::f64)?,
        status: daily.optional("status", decoder::decode::string)?,
        symbol: daily.optional("symbol", decoder::decode::string)?,
        volume: daily.optional("volume", decoder::decode::f64)?,
        otc: daily.optional("otc", decoder::decode::bool)?,
    })
}
