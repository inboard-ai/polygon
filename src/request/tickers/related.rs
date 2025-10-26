use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::client::Polygon;
use crate::error::Result;
use crate::execute::Execute;
use crate::processor::{Decoder, Processor, Raw};
use crate::request::Request;

/// Request builder for related tickers
pub struct Related<'a, Client: Request, P: Processor = Raw> {
    client: &'a Polygon<Client>,
    /// Ticker symbol (e.g., "AAPL" for Apple Inc.)
    pub ticker: String,
    processor: P,
}

impl<'a, C: Request> Related<'a, C, Raw> {
    /// Create a new related tickers request
    pub fn new(client: &'a Polygon<C>, ticker: impl Into<String>) -> Self {
        Self {
            client,
            ticker: ticker.into(),
            processor: Raw,
        }
    }
}

impl<'a, C: Request, P: Processor + 'a> Related<'a, C, P> {
    /// Execute the request and return the result
    pub fn get(self) -> impl std::future::Future<Output = Result<P::Output>> + 'a {
        Execute::get(self)
    }

    /// Convert this request to return results as a Polars DataFrame instead of raw JSON
    #[cfg(feature = "table")]
    pub fn as_dataframe(self) -> Related<'a, C, crate::processor::Table> {
        Related {
            client: self.client,
            ticker: self.ticker,
            processor: crate::processor::Table,
        }
    }

    /// Convert to decoded typed output (`Vec<String>`)
    pub fn decoded(self) -> Related<'a, C, Decoder<Vec<String>>> {
        use crate::rest::decoded::tickers::decode;
        let decoder = Decoder::new(decode::tickers);

        Related {
            client: self.client,
            ticker: self.ticker,
            processor: decoder,
        }
    }

    /// Set a custom decoder function to convert JSON to typed data
    pub fn with_decoder<T>(
        self,
        decoder_fn: impl Fn(decoder::Value) -> decoder::Result<T> + Send + Sync + 'static,
    ) -> Related<'a, C, Decoder<T>> {
        Related {
            client: self.client,
            ticker: self.ticker,
            processor: Decoder::new(decoder_fn),
        }
    }
}

impl<'a, C: Request, P: Processor + 'a> Execute for Related<'a, C, P> {
    type Output = P::Output;

    #[allow(refining_impl_trait_reachable)]
    async fn get(self) -> Result<P::Output> {
        let api_key = self
            .client
            .api_key()
            .ok_or_else(|| crate::error::Error::Custom("API key not set".to_string()))?;

        let path = format!("/v1/related-companies/{}?apiKey={}", self.ticker, api_key);
        let url = format!("https://api.polygon.io{path}");
        let response = self.client.client().get(&url).await;
        self.processor.process(response)
    }
}

/// JSON-serializable parameters for related tickers request
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Params {
    /// Ticker symbol (e.g., "AAPL" for Apple Inc.)
    pub ticker: String,
}
