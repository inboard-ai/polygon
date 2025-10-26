use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::client::Polygon;
use crate::error::Result;
use crate::execute::Execute;
use crate::processor::{Decoder, Processor, Raw};
use crate::request::Request;
use crate::response::ticker::Ticker;

/// Request builder for ticker details (overview)
pub struct Details<'a, Client: Request, P: Processor = Raw> {
    client: &'a Polygon<Client>,
    /// Ticker symbol (e.g., "AAPL" for Apple Inc.)
    pub ticker: String,
    /// Date to retrieve details for (YYYY-MM-DD)
    pub date: Option<String>,
    processor: P,
}

impl<'a, C: Request> Details<'a, C, Raw> {
    /// Create a new ticker details request
    pub fn new(client: &'a Polygon<C>, ticker: impl Into<String>) -> Self {
        Self {
            client,
            ticker: ticker.into(),
            date: None,
            processor: Raw,
        }
    }
}

impl<'a, C: Request, P: Processor + 'a> Details<'a, C, P> {
    /// Execute the request and return the result
    pub fn get(self) -> impl std::future::Future<Output = Result<P::Output>> + 'a {
        Execute::get(self)
    }

    /// Convert this request to return results as a Polars DataFrame instead of raw JSON
    #[cfg(feature = "table")]
    pub fn as_dataframe(self) -> Details<'a, C, crate::processor::Table> {
        Details {
            client: self.client,
            ticker: self.ticker,
            date: self.date,
            processor: crate::processor::Table,
        }
    }

    /// Convert to decoded typed output (Ticker)
    pub fn decoded(self) -> Details<'a, C, Decoder<Ticker>> {
        use crate::rest::decoded::tickers::decode;
        let decoder = Decoder::new(decode::details);

        Details {
            client: self.client,
            ticker: self.ticker,
            date: self.date,
            processor: decoder,
        }
    }

    /// Set a custom decoder function to convert JSON to typed data
    pub fn with_decoder<T>(
        self,
        decoder_fn: impl Fn(decoder::Value) -> decoder::Result<T> + Send + Sync + 'static,
    ) -> Details<'a, C, Decoder<T>> {
        Details {
            client: self.client,
            ticker: self.ticker,
            date: self.date,
            processor: Decoder::new(decoder_fn),
        }
    }

    /// Set the date to retrieve details for
    pub fn date(mut self, d: impl Into<String>) -> Self {
        self.date = Some(d.into());
        self
    }
}

impl<'a, C: Request, P: Processor + 'a> Execute for Details<'a, C, P> {
    type Output = P::Output;

    #[allow(refining_impl_trait_reachable)]
    async fn get(self) -> Result<P::Output> {
        let api_key = self
            .client
            .api_key()
            .ok_or_else(|| crate::error::Error::Custom("API key not set".to_string()))?;

        let mut path = format!("/v3/reference/tickers/{}", self.ticker);
        let mut params = vec![format!("apiKey={}", api_key)];

        if let Some(d) = self.date {
            params.push(format!("date={d}"));
        }

        path.push('?');
        path.push_str(&params.join("&"));
        let url = format!("https://api.polygon.io{path}");
        let response = self.client.client().get(&url).await;
        self.processor.process(response)
    }
}

/// JSON-serializable parameters for ticker details request
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Params {
    /// Ticker symbol (e.g., "AAPL" for Apple Inc.)
    pub ticker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date to retrieve details for (YYYY-MM-DD)
    pub date: Option<String>,
}
