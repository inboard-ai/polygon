use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::client::Polygon;
use crate::error::Result;
use crate::execute::Execute;
use crate::processor::{Decoder, Processor, Raw};
use crate::request::Request;
use crate::response::aggs::DailyOpenCloseAgg;

/// Daily open/close request builder
pub struct DailyOpenClose<'a, Client: Request, P: Processor = Raw> {
    client: &'a Polygon<Client>,
    /// Ticker symbol (e.g., "AAPL" for Apple Inc.)
    pub ticker: String,
    /// Date to get open/close data for (YYYY-MM-DD)
    pub date: String,
    /// Whether results are adjusted for splits (default: true)
    pub adjusted: Option<bool>,
    processor: P,
}

impl<'a, C: Request> DailyOpenClose<'a, C, Raw> {
    /// Create new daily open/close request
    pub fn new(client: &'a Polygon<C>, ticker: impl Into<String>, date: impl Into<String>) -> Self {
        Self {
            client,
            ticker: ticker.into(),
            date: date.into(),
            adjusted: None,
            processor: Raw,
        }
    }
}

impl<'a, C: Request, P: Processor + 'a> DailyOpenClose<'a, C, P> {
    /// Execute the request and return the result
    pub fn get(self) -> impl std::future::Future<Output = Result<P::Output>> + 'a {
        Execute::get(self)
    }

    /// Convert to DataFrame output
    #[cfg(feature = "table")]
    pub fn as_dataframe(self) -> DailyOpenClose<'a, C, crate::processor::Table> {
        DailyOpenClose {
            client: self.client,
            ticker: self.ticker,
            date: self.date,
            adjusted: self.adjusted,
            processor: crate::processor::Table,
        }
    }

    /// Convert to decoded typed output (DailyOpenCloseAgg)
    pub fn decoded(self) -> DailyOpenClose<'a, C, Decoder<DailyOpenCloseAgg>> {
        use decoder::decode::{bool as bool_decode, f64, map, string};

        let decoder = Decoder::new(|value: decoder::Value| {
            let mut daily = map(value)?;
            Ok(DailyOpenCloseAgg {
                after_hours: daily.optional("afterHours", f64)?,
                close: daily.optional("close", f64)?,
                from: daily.optional("from", string)?,
                high: daily.optional("high", f64)?,
                low: daily.optional("low", f64)?,
                open: daily.optional("open", f64)?,
                pre_market: daily.optional("preMarket", f64)?,
                status: daily.optional("status", string)?,
                symbol: daily.optional("symbol", string)?,
                volume: daily.optional("volume", f64)?,
                otc: daily.optional("otc", bool_decode)?,
            })
        });

        DailyOpenClose {
            client: self.client,
            ticker: self.ticker,
            date: self.date,
            adjusted: self.adjusted,
            processor: decoder,
        }
    }

    /// Set a custom decoder function
    pub fn with_decoder(
        self,
        decoder_fn: impl Fn(decoder::Value) -> decoder::Result<DailyOpenCloseAgg> + Send + Sync + 'static,
    ) -> DailyOpenClose<'a, C, Decoder<DailyOpenCloseAgg>> {
        DailyOpenClose {
            client: self.client,
            ticker: self.ticker,
            date: self.date,
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

impl<'a, C: Request, P: Processor + 'a> Execute for DailyOpenClose<'a, C, P> {
    type Output = P::Output;

    #[allow(refining_impl_trait_reachable)]
    async fn get(self) -> Result<P::Output> {
        let api_key = self
            .client
            .api_key()
            .ok_or_else(|| crate::error::Error::Custom("API key not set".to_string()))?;

        let mut path = format!(
            "/v2/aggs/ticker/{}/range/1/day/{}/{}",
            self.ticker, self.date, self.date
        );
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

/// JSON-serializable parameters for daily open/close request
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Params {
    /// Ticker symbol (e.g., "AAPL" for Apple Inc.)
    pub ticker: String,
    /// Date to get open/close data for (YYYY-MM-DD)
    pub date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether results are adjusted for splits (default: true)
    pub adjusted: Option<bool>,
}
