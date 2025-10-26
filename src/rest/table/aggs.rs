//! Aggregates endpoints returning Polars DataFrames
use crate::client::Polygon;
use crate::processor::Table;
use crate::request::Request;
use crate::request::aggs::{Aggregates, DailyOpenClose, GroupedDaily, PreviousClose};
use crate::request::common::Timespan;

/// Get aggregate bars for a stock over a given date range
///
/// Returns a request builder that will return results as a Polars DataFrame.
/// Use builder methods like `.adjusted()`, `.sort()`, `.limit()` to customize the request.
///
/// # Example
/// ```no_run
/// # use polygon::Client;
/// # use polygon::execute::Execute;
/// # async fn example() {
/// # let client = Client::new("api-key");
/// let df = polygon::rest::table::aggs::aggregates(&client, "AAPL", 1, "day", "2023-01-01", "2023-12-31")
///     .adjusted(true)
///     .get()
///     .await
///     .unwrap();
/// # }
/// ```
pub fn aggregates<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: impl Into<String>,
    multiplier: u32,
    timespan: impl Into<String>,
    from: impl Into<String>,
    to: impl Into<String>,
) -> Aggregates<'a, Client, Table> {
    let timespan_str = timespan.into();
    let timespan_enum = match timespan_str.to_lowercase().as_str() {
        "minute" => Timespan::Minute,
        "hour" => Timespan::Hour,
        "day" => Timespan::Day,
        "week" => Timespan::Week,
        "month" => Timespan::Month,
        "quarter" => Timespan::Quarter,
        "year" => Timespan::Year,
        _ => Timespan::Day, // Default fallback
    };

    Aggregates::new(client, ticker, multiplier, timespan_enum, from, to).as_dataframe()
}

/// Get the previous day's OHLC for a stock
///
/// Returns a request builder that will return results as a Polars DataFrame.
/// Use builder methods like `.adjusted()` to customize the request.
pub fn previous_close<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: impl Into<String>,
) -> PreviousClose<'a, Client, Table> {
    PreviousClose::new(client, ticker).as_dataframe()
}

/// Get daily OHLC for the entire market
///
/// Returns a request builder that will return results as a Polars DataFrame.
/// Use builder methods like `.adjusted()` and `.include_otc()` to customize the request.
pub fn grouped_daily<'a, Client: Request>(
    client: &'a Polygon<Client>,
    date: impl Into<String>,
) -> GroupedDaily<'a, Client, Table> {
    GroupedDaily::new(client, date).as_dataframe()
}

/// Get the open/close/afterhours prices of a stock on a specific date
///
/// Returns a request builder that will return results as a Polars DataFrame.
/// Use builder methods like `.adjusted()` to customize the request.
pub fn daily_open_close<'a, Client: Request>(
    client: &'a Polygon<Client>,
    ticker: impl Into<String>,
    date: impl Into<String>,
) -> DailyOpenClose<'a, Client, Table> {
    DailyOpenClose::new(client, ticker, date).as_dataframe()
}
