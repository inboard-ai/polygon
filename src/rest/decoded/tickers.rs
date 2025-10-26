//! Decoded ticker endpoints - returns typed data instead of JSON strings

use crate::client::Polygon;
use crate::processor::Decoder;
use crate::request::Request;
use crate::request::tickers::{All, Details, Events, News, Related};
use crate::rest::raw;

pub use crate::response::ticker::*;

/// Get a list of all tickers
pub fn all<'a, Client: Request>(client: &'a Polygon<Client>) -> All<'a, Client, Decoder<Vec<Ticker>>> {
    raw::tickers::all(client).with_decoder(decode::all)
}

/// Get detailed information about a ticker
pub fn details<'a, Client: Request>(client: &'a Polygon<Client>, ticker: &str) -> Details<'a, Client, Decoder<Ticker>> {
    raw::tickers::details(client, ticker).with_decoder(decode::details)
}

/// Get tickers related to a given ticker
pub fn related<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: &str,
) -> Related<'a, Client, Decoder<Vec<String>>> {
    raw::tickers::related(client, ticker).with_decoder(decode::tickers)
}

/// Get event history for a ticker at a particular point in time
pub fn events<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: &str,
) -> Events<'a, Client, Decoder<TickerChangeResults>> {
    raw::tickers::events(client, ticker).with_decoder(decode::events)
}

/// Get the most recent news articles relating to a stock ticker symbol
pub fn news<'a, Client: Request>(client: &'a Polygon<Client>) -> News<'a, Client, Decoder<Vec<TickerNews>>> {
    raw::tickers::news(client).with_decoder(decode::news)
}

pub mod decode {
    //! Decode functions for ticker endpoints
    use super::*;
    use decoder::decode::{bool, map, sequence, string};

    /// Decode a list of tickers
    pub fn all(value: decoder::Value) -> decoder::Result<Vec<Ticker>> {
        let mut response = map(value)?;
        response.required("results", sequence(ticker))
    }

    /// Decode ticker details
    pub fn details(value: decoder::Value) -> decoder::Result<Ticker> {
        let mut response = map(value)?;
        response.required("results", ticker)
    }

    /// Decode a list of tickers
    pub fn tickers(value: decoder::Value) -> decoder::Result<Vec<String>> {
        let mut response = map(value)?;
        response.required(
            "results",
            sequence(|v| {
                let mut obj = map(v)?;
                obj.required("ticker", string)
            }),
        )
    }

    /// Decode a ticker
    pub fn ticker(value: decoder::Value) -> decoder::Result<Ticker> {
        let mut ticker = map(value)?;

        Ok(Ticker {
            active: ticker.optional("active", bool)?,
            cik: ticker.optional("cik", string)?,
            composite_figi: ticker.optional("composite_figi", string)?,
            currency_name: ticker.optional("currency_name", string)?,
            currency_symbol: ticker.optional("currency_symbol", string)?,
            base_currency_symbol: ticker.optional("base_currency_symbol", string)?,
            base_currency_name: ticker.optional("base_currency_name", string)?,
            delisted_utc: ticker.optional("delisted_utc", string)?,
            last_updated_utc: ticker.optional("last_updated_utc", string)?,
            locale: ticker.optional("locale", string)?,
            market: ticker.optional("market", string)?,
            name: ticker.optional("name", string)?,
            primary_exchange: ticker.optional("primary_exchange", string)?,
            share_class_figi: ticker.optional("share_class_figi", string)?,
            ticker: ticker.optional("ticker", string)?,
            type_: ticker.optional("type", string)?,
            source_feed: ticker.optional("source_feed", string)?,
        })
    }

    /// Decode ticker events
    pub fn events(value: decoder::Value) -> decoder::Result<TickerChangeResults> {
        let mut response = map(value)?;
        response.required("results", ticker_change_results)
    }

    /// Decode a list of news articles
    pub fn news(value: decoder::Value) -> decoder::Result<Vec<TickerNews>> {
        let mut response = map(value)?;
        response.required("results", sequence(ticker_news))
    }

    /// Decode ticker change results
    pub fn ticker_change_results(value: decoder::Value) -> decoder::Result<TickerChangeResults> {
        let mut obj = map(value)?;

        Ok(TickerChangeResults {
            name: obj.required("name", string)?,
            composite_figi: obj.required("composite_figi", string)?,
            cik: obj.required("cik", string)?,
            events: obj.optional("events", sequence(ticker_change_event))?,
        })
    }

    /// Decode a ticker change event
    pub fn ticker_change_event(value: decoder::Value) -> decoder::Result<TickerChangeEvent> {
        let mut obj = map(value)?;

        Ok(TickerChangeEvent {
            event_type: obj.required("type", string)?,
            date: obj.required("date", string)?,
            ticker_change: obj.required("ticker_change", ticker_change)?,
        })
    }

    /// Decode a ticker change
    pub fn ticker_change(value: decoder::Value) -> decoder::Result<TickerChange> {
        let mut obj = map(value)?;

        Ok(TickerChange {
            ticker: obj.required("ticker", string)?,
        })
    }

    /// Decode a news article
    pub fn ticker_news(value: decoder::Value) -> decoder::Result<TickerNews> {
        let mut obj = map(value)?;

        Ok(TickerNews {
            amp_url: obj.optional("amp_url", string)?,
            article_url: obj.optional("article_url", string)?,
            author: obj.optional("author", string)?,
            description: obj.optional("description", string)?,
            id: obj.optional("id", string)?,
            image_url: obj.optional("image_url", string)?,
            insights: obj.optional("insights", sequence(insight))?,
            keywords: obj.optional("keywords", sequence(string))?,
            published_utc: obj.optional("published_utc", string)?,
            publisher: obj.optional("publisher", publisher)?,
            tickers: obj.optional("tickers", sequence(string))?,
            title: obj.optional("title", string)?,
        })
    }

    /// Decode a publisher
    pub fn publisher(value: decoder::Value) -> decoder::Result<Publisher> {
        let mut obj = map(value)?;

        Ok(Publisher {
            favicon_url: obj.optional("favicon_url", string)?,
            homepage_url: obj.optional("homepage_url", string)?,
            logo_url: obj.optional("logo_url", string)?,
            name: obj.optional("name", string)?,
        })
    }

    /// Decode an insight
    pub fn insight(value: decoder::Value) -> decoder::Result<Insight> {
        let mut obj = map(value)?;

        Ok(Insight {
            sentiment: obj.optional("sentiment", string)?,
            sentiment_reasoning: obj.optional("sentiment_reasoning", string)?,
            ticker: obj.optional("ticker", string)?,
        })
    }
}
