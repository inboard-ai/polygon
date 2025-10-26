use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::client::Polygon;
use crate::error::Result;
use crate::execute::Execute;
use crate::processor::{Decoder, Processor, Raw};
use crate::request::Request;
use crate::response::aggs::PreviousCloseAgg;

/// Previous close request builder
pub struct PreviousClose<'a, Client: Request, P: Processor = Raw> {
    client: &'a Polygon<Client>,
    /// Ticker symbol (e.g., "AAPL" for Apple Inc.)
    pub ticker: String,
    /// Whether results are adjusted for splits (default: true)
    pub adjusted: Option<bool>,
    processor: P,
}

impl<'a, C: Request> PreviousClose<'a, C, Raw> {
    /// Create new previous close request
    pub fn new(client: &'a Polygon<C>, ticker: impl Into<String>) -> Self {
        Self {
            client,
            ticker: ticker.into(),
            adjusted: None,
            processor: Raw,
        }
    }
}

impl<'a, C: Request, P: Processor + 'a> PreviousClose<'a, C, P> {
    /// Execute the request and return the result
    pub fn get(self) -> impl std::future::Future<Output = Result<P::Output>> + 'a {
        Execute::get(self)
    }

    /// Convert to DataFrame output
    #[cfg(feature = "table")]
    pub fn as_dataframe(self) -> PreviousClose<'a, C, crate::processor::Table> {
        PreviousClose {
            client: self.client,
            ticker: self.ticker,
            adjusted: self.adjusted,
            processor: crate::processor::Table,
        }
    }

    /// Convert to decoded typed output (`Vec<`[`PreviousCloseAgg`]`>`)
    pub fn decoded(self) -> PreviousClose<'a, C, Decoder<Vec<PreviousCloseAgg>>> {
        use decoder::decode::{f64, i64, map, sequence, string};

        let decoder = Decoder::new(|value: decoder::Value| {
            let mut response = map(value)?;
            response.required(
                "results",
                sequence(|v| {
                    let mut prev = map(v)?;
                    Ok(PreviousCloseAgg {
                        ticker: prev.optional("T", string)?,
                        close: prev.optional("c", f64)?,
                        high: prev.optional("h", f64)?,
                        low: prev.optional("l", f64)?,
                        open: prev.optional("o", f64)?,
                        timestamp: prev.optional("t", i64)?,
                        volume: prev.optional("v", f64)?,
                        vwap: prev.optional("vw", f64)?,
                    })
                }),
            )
        });

        PreviousClose {
            client: self.client,
            ticker: self.ticker,
            adjusted: self.adjusted,
            processor: decoder,
        }
    }

    /// Set a custom decoder function
    pub fn with_decoder(
        self,
        decoder_fn: impl Fn(decoder::Value) -> decoder::Result<Vec<PreviousCloseAgg>> + Send + Sync + 'static,
    ) -> PreviousClose<'a, C, Decoder<Vec<PreviousCloseAgg>>> {
        PreviousClose {
            client: self.client,
            ticker: self.ticker,
            adjusted: self.adjusted,
            processor: Decoder::new(decoder_fn),
        }
    }

    /// Set adjusted flag
    pub fn adjusted(mut self, adjusted: bool) -> Self {
        self.adjusted = Some(adjusted);
        self
    }
}

impl<'a, C: Request, P: Processor + 'a> Execute for PreviousClose<'a, C, P> {
    type Output = P::Output;

    #[allow(refining_impl_trait_reachable)]
    async fn get(self) -> Result<P::Output> {
        let api_key = self
            .client
            .api_key()
            .ok_or_else(|| crate::error::Error::Custom("API key not set".to_string()))?;

        let mut path = format!("/v2/aggs/ticker/{}/prev", self.ticker);
        let mut params = vec![format!("apiKey={}", api_key)];

        if let Some(a) = self.adjusted {
            params.push(format!("adjusted={a}"));
        }

        path.push('?');
        path.push_str(&params.join("&"));
        let url = format!("https://api.polygon.io{path}");
        let response = self.client.client().get(&url).await;
        self.processor.process(response)
    }
}

/// JSON-serializable parameters for previous close request
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Params {
    /// Ticker symbol (e.g., "AAPL" for Apple Inc.)
    pub ticker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether results are adjusted for splits (default: true)
    pub adjusted: Option<bool>,
}
