use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::client::Polygon;
use crate::error::Result;
use crate::execute::Execute;
use crate::processor::{Decoder, Processor, Raw};
use crate::request::Request;
use crate::response::ticker::TickerChangeResults;

/// Request builder for ticker events
pub struct Events<'a, Client: Request, P: Processor = Raw> {
    client: &'a Polygon<Client>,
    /// Ticker symbol (e.g., "AAPL" for Apple Inc.)
    pub ticker: String,
    /// Event types to filter by (comma-separated)
    pub types: Option<String>,
    processor: P,
}

impl<'a, C: Request> Events<'a, C, Raw> {
    /// Create a new ticker events request
    pub fn new(client: &'a Polygon<C>, ticker: impl Into<String>) -> Self {
        Self {
            client,
            ticker: ticker.into(),
            types: None,
            processor: Raw,
        }
    }
}

impl<'a, C: Request, P: Processor + 'a> Events<'a, C, P> {
    /// Execute the request and return the result
    pub fn get(self) -> impl std::future::Future<Output = Result<P::Output>> + 'a {
        Execute::get(self)
    }

    /// Convert this request to return results as a Polars DataFrame instead of raw JSON
    #[cfg(feature = "table")]
    pub fn as_dataframe(self) -> Events<'a, C, crate::processor::Table> {
        Events {
            client: self.client,
            ticker: self.ticker,
            types: self.types,
            processor: crate::processor::Table,
        }
    }

    /// Convert to decoded typed output (TickerChangeResults)
    pub fn decoded(self) -> Events<'a, C, Decoder<TickerChangeResults>> {
        use crate::rest::decoded::tickers::decode;
        let decoder = Decoder::new(decode::events);

        Events {
            client: self.client,
            ticker: self.ticker,
            types: self.types,
            processor: decoder,
        }
    }

    /// Set a custom decoder function to convert JSON to typed data
    pub fn with_decoder<T>(
        self,
        decoder_fn: impl Fn(decoder::Value) -> decoder::Result<T> + Send + Sync + 'static,
    ) -> Events<'a, C, Decoder<T>> {
        Events {
            client: self.client,
            ticker: self.ticker,
            types: self.types,
            processor: Decoder::new(decoder_fn),
        }
    }

    /// Set the event types to filter by (comma-separated)
    pub fn types(mut self, t: impl Into<String>) -> Self {
        self.types = Some(t.into());
        self
    }
}

impl<'a, C: Request, P: Processor + 'a> Execute for Events<'a, C, P> {
    type Output = P::Output;

    #[allow(refining_impl_trait_reachable)]
    async fn get(self) -> Result<P::Output> {
        let api_key = self
            .client
            .api_key()
            .ok_or_else(|| crate::error::Error::Custom("API key not set".to_string()))?;

        let mut path = format!("/vX/reference/tickers/{}/events", self.ticker);
        let mut params = vec![format!("apiKey={}", api_key)];

        if let Some(t) = self.types {
            params.push(format!("types={t}"));
        }

        path.push('?');
        path.push_str(&params.join("&"));
        let url = format!("https://api.polygon.io{path}");
        let response = self.client.client().get(&url).await;
        self.processor.process(response)
    }
}

/// JSON-serializable parameters for ticker events request
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct Params {
    /// Ticker symbol (e.g., "AAPL" for Apple Inc.)
    pub ticker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Event types to filter by (comma-separated)
    pub types: Option<String>,
}
