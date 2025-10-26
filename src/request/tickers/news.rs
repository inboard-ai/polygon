use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::client::Polygon;
use crate::error::Result;
use crate::execute::Execute;
use crate::processor::{Decoder, Processor, Raw};
use crate::request::Request;
use crate::request::common::{Limit, SortOrder};
use crate::response::ticker::TickerNews;

/// Request builder for news articles
pub struct News<'a, Client: Request, P: Processor = Raw> {
    client: &'a Polygon<Client>,
    /// Ticker symbol to filter news by (e.g., "AAPL" for Apple Inc.)
    pub ticker: Option<String>,
    /// Maximum number of results to return
    pub limit: Option<u32>,
    /// Sort order for results (asc or desc)
    pub order: Option<SortOrder>,
    processor: P,
}

impl<'a, C: Request> News<'a, C, Raw> {
    /// Create a new news request
    pub fn new(client: &'a Polygon<C>) -> Self {
        Self {
            client,
            ticker: None,
            limit: None,
            order: None,
            processor: Raw,
        }
    }
}

impl<'a, C: Request, P: Processor + 'a> News<'a, C, P> {
    /// Execute the request and return the result
    pub fn get(self) -> impl std::future::Future<Output = Result<P::Output>> + 'a {
        Execute::get(self)
    }

    /// Convert this request to return results as a Polars DataFrame instead of raw JSON
    #[cfg(feature = "table")]
    pub fn as_dataframe(self) -> News<'a, C, crate::processor::Table> {
        News {
            client: self.client,
            ticker: self.ticker,
            limit: self.limit,
            order: self.order,
            processor: crate::processor::Table,
        }
    }

    /// Convert to decoded typed output (`Vec<`[`TickerNews`]`>`)
    pub fn decoded(self) -> News<'a, C, Decoder<Vec<TickerNews>>> {
        use crate::rest::decoded::tickers::decode;
        let decoder = Decoder::new(decode::news);

        News {
            client: self.client,
            ticker: self.ticker,
            limit: self.limit,
            order: self.order,
            processor: decoder,
        }
    }

    /// Set a custom decoder function to convert JSON to typed data
    pub fn with_decoder<T>(
        self,
        decoder_fn: impl Fn(decoder::Value) -> decoder::Result<T> + Send + Sync + 'static,
    ) -> News<'a, C, Decoder<T>> {
        News {
            client: self.client,
            ticker: self.ticker,
            limit: self.limit,
            order: self.order,
            processor: Decoder::new(decoder_fn),
        }
    }

    /// Filter news by ticker symbol
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    /// Set the maximum number of results to return
    pub fn limit(mut self, limit: impl Into<Limit>) -> Self {
        self.limit = limit.into().into();
        self
    }

    /// Set the sort order (asc or desc)
    pub fn order(mut self, order: SortOrder) -> Self {
        self.order = Some(order);
        self
    }
}

impl<'a, C: Request, P: Processor + 'a> Execute for News<'a, C, P> {
    type Output = P::Output;

    #[allow(refining_impl_trait_reachable)]
    async fn get(self) -> Result<P::Output> {
        let api_key = self
            .client
            .api_key()
            .ok_or_else(|| crate::error::Error::Custom("API key not set".to_string()))?;

        let mut path = "/v2/reference/news".to_string();
        let mut params = vec![format!("apiKey={}", api_key)];

        if let Some(t) = self.ticker {
            params.push(format!("ticker={t}"));
        }
        if let Some(l) = self.limit {
            params.push(format!("limit={l}"));
        }
        if let Some(o) = self.order {
            params.push(format!("order={}", format!("{o:?}").to_lowercase()));
        }

        path.push('?');
        path.push_str(&params.join("&"));
        let url = format!("https://api.polygon.io{path}");
        let response = self.client.client().get(&url).await;
        self.processor.process(response)
    }
}

/// JSON-serializable parameters for news request
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct Params {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Ticker symbol to filter news by (e.g., "AAPL" for Apple Inc.")
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum number of results to return
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sort order for results (asc or desc)
    pub order: Option<SortOrder>,
}
