use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::client::Polygon;
use crate::error::Result;
use crate::execute::Execute;
use crate::processor::{Decoder, Processor, Raw};
use crate::request::Request;
use crate::response::aggs::GroupedDailyAgg;

/// Grouped daily bars request builder
pub struct GroupedDaily<'a, Client: Request, P: Processor = Raw> {
    client: &'a Polygon<Client>,
    /// Date to get grouped daily bars for (YYYY-MM-DD)
    pub date: String,
    /// Whether results are adjusted for splits (default: true)
    pub adjusted: Option<bool>,
    /// Include OTC (Over-the-Counter) securities in the results
    pub include_otc: Option<bool>,
    processor: P,
}

impl<'a, C: Request> GroupedDaily<'a, C, Raw> {
    /// Create new grouped daily request
    pub fn new(client: &'a Polygon<C>, date: impl Into<String>) -> Self {
        Self {
            client,
            date: date.into(),
            adjusted: None,
            include_otc: None,
            processor: Raw,
        }
    }
}

impl<'a, C: Request, P: Processor + 'a> GroupedDaily<'a, C, P> {
    /// Execute the request and return the result
    pub fn get(self) -> impl std::future::Future<Output = Result<P::Output>> + 'a {
        Execute::get(self)
    }

    /// Convert to DataFrame output
    #[cfg(feature = "table")]
    pub fn as_dataframe(self) -> GroupedDaily<'a, C, crate::processor::Table> {
        GroupedDaily {
            client: self.client,
            date: self.date,
            adjusted: self.adjusted,
            include_otc: self.include_otc,
            processor: crate::processor::Table,
        }
    }

    /// Convert to decoded typed output (`Vec<`[`GroupedDailyAgg`]`>`)
    pub fn decoded(self) -> GroupedDaily<'a, C, Decoder<Vec<GroupedDailyAgg>>> {
        use decoder::decode::{bool as bool_decode, f64, i64, map, sequence, string};

        let decoder = Decoder::new(|value: decoder::Value| {
            let mut response = map(value)?;
            response.required(
                "results",
                sequence(|v| {
                    let mut grouped = map(v)?;
                    Ok(GroupedDailyAgg {
                        ticker: grouped.optional("T", string)?,
                        open: grouped.optional("o", f64)?,
                        high: grouped.optional("h", f64)?,
                        low: grouped.optional("l", f64)?,
                        close: grouped.optional("c", f64)?,
                        volume: grouped.optional("v", f64)?,
                        vwap: grouped.optional("vw", f64)?,
                        timestamp: grouped.optional("t", i64)?,
                        transactions: grouped.optional("n", i64)?,
                        otc: grouped.optional("otc", bool_decode)?,
                    })
                }),
            )
        });

        GroupedDaily {
            client: self.client,
            date: self.date,
            adjusted: self.adjusted,
            include_otc: self.include_otc,
            processor: decoder,
        }
    }

    /// Set a custom decoder function
    pub fn with_decoder(
        self,
        decoder_fn: impl Fn(decoder::Value) -> decoder::Result<Vec<GroupedDailyAgg>> + Send + Sync + 'static,
    ) -> GroupedDaily<'a, C, Decoder<Vec<GroupedDailyAgg>>> {
        GroupedDaily {
            client: self.client,
            date: self.date,
            adjusted: self.adjusted,
            include_otc: self.include_otc,
            processor: Decoder::new(decoder_fn),
        }
    }

    /// Set adjusted flag
    pub fn adjusted(mut self, adjusted: bool) -> Self {
        self.adjusted = Some(adjusted);
        self
    }

    /// Set include_otc flag
    pub fn include_otc(mut self, include_otc: bool) -> Self {
        self.include_otc = Some(include_otc);
        self
    }
}

impl<'a, C: Request, P: Processor + 'a> Execute for GroupedDaily<'a, C, P> {
    type Output = P::Output;

    #[allow(refining_impl_trait_reachable)]
    async fn get(self) -> Result<P::Output> {
        let api_key = self
            .client
            .api_key()
            .ok_or_else(|| crate::error::Error::Custom("API key not set".to_string()))?;

        let mut path = format!("/v2/aggs/grouped/locale/us/market/stocks/{}", self.date);
        let mut params = vec![format!("apiKey={}", api_key)];

        if let Some(a) = self.adjusted {
            params.push(format!("adjusted={a}"));
        }
        if let Some(o) = self.include_otc {
            params.push(format!("include_otc={o}"));
        }

        path.push('?');
        path.push_str(&params.join("&"));
        let url = format!("https://api.polygon.io{path}");
        let response = self.client.client().get(&url).await;
        self.processor.process(response)
    }
}

/// JSON-serializable parameters for grouped daily bars request
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Params {
    /// Date to get grouped daily bars for (YYYY-MM-DD)
    pub date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether results are adjusted for splits (default: true)
    pub adjusted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Include OTC (Over-the-Counter) securities in the results
    pub include_otc: Option<bool>,
}
