//! Decoded ticker endpoints - returns typed data instead of JSON strings

use crate::client::Polygon;
use crate::query::Query;
use crate::request::Request;
use crate::rest::raw;

pub use crate::schema::ticker::*;

/// Get a list of all tickers
pub fn all<'a, Client: Request>(client: &'a Polygon<Client>) -> Query<'a, Client, Vec<Ticker>> {
    raw::tickers::all(client).with_decoder(_all)
}

/// Get detailed information about a ticker
pub fn details<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: &str,
) -> Query<'a, Client, Ticker> {
    raw::tickers::details(client, ticker).with_decoder(_details)
}

/// Get tickers related to a given ticker
pub fn related<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: &str,
) -> Query<'a, Client, Vec<String>> {
    raw::tickers::related(client, ticker).with_decoder(_tickers)
}

// Internal decoder functions
fn _all(value: decoder::Value) -> decoder::Result<Vec<Ticker>> {
    let mut response = decoder::decode::map(value)?;
    response.required("results", decoder::decode::sequence(_ticker))
}

fn _details(value: decoder::Value) -> decoder::Result<Ticker> {
    let mut response = decoder::decode::map(value)?;
    response.required("results", _ticker)
}

fn _tickers(value: decoder::Value) -> decoder::Result<Vec<String>> {
    let mut response = decoder::decode::map(value)?;
    response.required(
        "results",
        decoder::decode::sequence(|v| {
            let mut obj = decoder::decode::map(v)?;
            obj.required("ticker", decoder::decode::string)
        }),
    )
}

fn _ticker(value: decoder::Value) -> decoder::Result<Ticker> {
    let mut ticker = decoder::decode::map(value)?;

    Ok(Ticker {
        active: ticker.optional("active", decoder::decode::bool)?,
        cik: ticker.optional("cik", decoder::decode::string)?,
        composite_figi: ticker.optional("composite_figi", decoder::decode::string)?,
        currency_name: ticker.optional("currency_name", decoder::decode::string)?,
        currency_symbol: ticker.optional("currency_symbol", decoder::decode::string)?,
        base_currency_symbol: ticker.optional("base_currency_symbol", decoder::decode::string)?,
        base_currency_name: ticker.optional("base_currency_name", decoder::decode::string)?,
        delisted_utc: ticker.optional("delisted_utc", decoder::decode::string)?,
        last_updated_utc: ticker.optional("last_updated_utc", decoder::decode::string)?,
        locale: ticker.optional("locale", decoder::decode::string)?,
        market: ticker.optional("market", decoder::decode::string)?,
        name: ticker.optional("name", decoder::decode::string)?,
        primary_exchange: ticker.optional("primary_exchange", decoder::decode::string)?,
        share_class_figi: ticker.optional("share_class_figi", decoder::decode::string)?,
        ticker: ticker.optional("ticker", decoder::decode::string)?,
        type_: ticker.optional("type", decoder::decode::string)?,
        source_feed: ticker.optional("source_feed", decoder::decode::string)?,
    })
}
