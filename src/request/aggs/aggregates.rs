use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::client::Polygon;
use crate::error::Result;
use crate::execute::Execute;
#[cfg(feature = "table")]
use crate::processor::Table;
use crate::processor::{Decoder, Processor, Raw};
use crate::request::Request;
use crate::request::common::{Limit, SortOrder, Timespan};
use crate::response::aggs::Agg;

/// Aggregates request builder
pub struct Aggregates<'a, Client: Request, P: Processor = Raw> {
    client: &'a Polygon<Client>,
    /// Ticker symbol (e.g., "AAPL" for Apple Inc.)
    pub ticker: String,
    /// The size of the timespan multiplier
    pub multiplier: u32,
    /// The size of the time window (e.g., minute, hour, day, week, month, quarter, year)
    pub timespan: Timespan,
    /// Start of the aggregate time window (YYYY-MM-DD or millisecond timestamp)
    pub from: String,
    /// End of the aggregate time window (YYYY-MM-DD or millisecond timestamp)
    pub to: String,
    /// Whether results are adjusted for splits (default: true)
    pub adjusted: Option<bool>,
    /// Sort order for results (asc or desc)
    pub sort: Option<SortOrder>,
    /// Maximum number of base aggregates queried (max: 50000, default: 5000)
    pub limit: Option<u32>,
    processor: P,
}

// Constructor - always starts with Raw
impl<'a, C: Request> Aggregates<'a, C, Raw> {
    /// Create new aggregates request (returns raw JSON by default)
    pub fn new(
        client: &'a Polygon<C>,
        ticker: impl Into<String>,
        multiplier: u32,
        timespan: Timespan,
        from: impl Into<String>,
        to: impl Into<String>,
    ) -> Self {
        Self {
            client,
            ticker: ticker.into(),
            multiplier,
            timespan,
            from: from.into(),
            to: to.into(),
            adjusted: None,
            sort: None,
            limit: None,
            processor: Raw,
        }
    }
}

// Processor conversion and builder methods work on any processor type
impl<'a, C: Request, P: Processor + 'a> Aggregates<'a, C, P> {
    /// Execute the request and return the result
    pub fn get(self) -> impl std::future::Future<Output = Result<P::Output>> + 'a {
        Execute::get(self)
    }

    /// Convert to DataFrame output
    #[cfg(feature = "table")]
    pub fn as_dataframe(self) -> Aggregates<'a, C, Table> {
        Aggregates {
            client: self.client,
            ticker: self.ticker,
            multiplier: self.multiplier,
            timespan: self.timespan,
            from: self.from,
            to: self.to,
            adjusted: self.adjusted,
            sort: self.sort,
            limit: self.limit,
            processor: Table,
        }
    }

    /// Convert to decoded typed output (`Vec<`[`Agg`]`>`)
    pub fn decoded(self) -> Aggregates<'a, C, Decoder<Vec<Agg>>> {
        use decoder::decode::{bool as bool_decode, f64, i64, map, sequence};

        // Default decoder for Vec<Agg>
        let decoder = Decoder::new(|value: decoder::Value| {
            let mut response = map(value)?;
            response.required(
                "results",
                sequence(|v| {
                    let mut agg = map(v)?;
                    Ok(Agg {
                        open: agg.optional("o", f64)?,
                        high: agg.optional("h", f64)?,
                        low: agg.optional("l", f64)?,
                        close: agg.optional("c", f64)?,
                        volume: agg.optional("v", f64)?,
                        vwap: agg.optional("vw", f64)?,
                        timestamp: agg.optional("t", i64)?,
                        transactions: agg.optional("n", i64)?,
                        otc: agg.optional("otc", bool_decode)?,
                    })
                }),
            )
        });

        Aggregates {
            client: self.client,
            ticker: self.ticker,
            multiplier: self.multiplier,
            timespan: self.timespan,
            from: self.from,
            to: self.to,
            adjusted: self.adjusted,
            sort: self.sort,
            limit: self.limit,
            processor: decoder,
        }
    }

    /// Set a custom decoder function
    pub fn with_decoder(
        self,
        decoder_fn: impl Fn(decoder::Value) -> decoder::Result<Vec<Agg>> + Send + Sync + 'static,
    ) -> Aggregates<'a, C, Decoder<Vec<Agg>>> {
        Aggregates {
            client: self.client,
            ticker: self.ticker,
            multiplier: self.multiplier,
            timespan: self.timespan,
            from: self.from,
            to: self.to,
            adjusted: self.adjusted,
            sort: self.sort,
            limit: self.limit,
            processor: Decoder::new(decoder_fn),
        }
    }

    /// Set adjusted flag
    pub fn adjusted(mut self, adjusted: bool) -> Self {
        self.adjusted = Some(adjusted);
        self
    }

    /// Set sort order
    pub fn sort(mut self, order: impl Into<SortOrder>) -> Self {
        self.sort = Some(order.into());
        self
    }

    /// Set limit
    pub fn limit(mut self, limit: impl Into<Limit>) -> Self {
        self.limit = limit.into().into();
        self
    }
}

impl<'a, C: Request, P: Processor + 'a> Execute for Aggregates<'a, C, P> {
    type Output = P::Output;

    #[allow(refining_impl_trait_reachable)]
    async fn get(self) -> Result<P::Output> {
        // Build URL path
        let timespan = format!("{:?}", self.timespan).to_lowercase();
        let mut path = format!(
            "/v2/aggs/ticker/{}/range/{}/{}/{}/{}",
            self.ticker, self.multiplier, timespan, self.from, self.to
        );

        // Add query params
        let api_key = self
            .client
            .api_key()
            .ok_or_else(|| crate::error::Error::Custom("API key not set".to_string()))?;

        let mut params = vec![format!("apiKey={}", api_key)];
        if let Some(a) = self.adjusted {
            params.push(format!("adjusted={a}"));
        }
        if let Some(s) = self.sort {
            params.push(format!("sort={}", format!("{s:?}").to_lowercase()));
        }
        if let Some(l) = self.limit {
            params.push(format!("limit={l}"));
        }

        path.push('?');
        path.push_str(&params.join("&"));

        // Build full URL
        let url = format!("https://api.polygon.io{path}");

        // Make request using Request trait
        let response = self.client.client().get(&url).await;

        // Process using associated Processor type
        self.processor.process(response)
    }
}

/// JSON-serializable parameters (no client reference, for tool_use)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Params {
    /// Ticker symbol (e.g., "AAPL" for Apple Inc.)
    pub ticker: String,
    /// The size of the timespan multiplier
    pub multiplier: u32,
    /// The size of the time window (e.g., minute, hour, day, week, month, quarter, year)
    pub timespan: Timespan,
    /// Start of the aggregate time window (YYYY-MM-DD or millisecond timestamp)
    pub from: String,
    /// End of the aggregate time window (YYYY-MM-DD or millisecond timestamp)
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether results are adjusted for splits (default: true)
    pub adjusted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sort order for results (asc or desc)
    pub sort: Option<SortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum number of base aggregates queried (max: 50000, default: 5000)
    pub limit: Option<u32>,
}
