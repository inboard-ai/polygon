use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::client::Polygon;
use crate::error::Result;
use crate::execute::Execute;
use crate::processor::{Decoder, Processor, Raw};
use crate::request::Request;
use crate::request::common::{Limit, SortOrder};
use crate::response::ticker::Ticker;

/// Request builder for querying all tickers
pub struct All<'a, Client: Request, P: Processor = Raw> {
    client: &'a Polygon<Client>,
    /// Ticker symbol to filter by (e.g., "AAPL" for Apple Inc.)
    pub ticker: Option<String>,
    /// Type of ticker (e.g., "CS" for common stock, "ETF" for exchange-traded fund)
    pub ticker_type: Option<String>,
    /// Market type (e.g., "stocks", "crypto", "fx")
    pub market: Option<String>,
    /// Primary exchange MIC (Market Identifier Code) according to ISO 10383
    pub exchange: Option<String>,
    /// Maximum number of results to return (default: 100, max: 1000)
    pub limit: Option<u32>,
    /// Field to sort by
    pub sort: Option<String>,
    /// Sort order for results (asc or desc)
    pub order: Option<SortOrder>,
    processor: P,
}

impl<'a, C: Request> All<'a, C, Raw> {
    /// Create a new all tickers request
    pub fn new(client: &'a Polygon<C>) -> Self {
        Self {
            client,
            ticker: None,
            ticker_type: None,
            market: None,
            exchange: None,
            limit: None,
            sort: None,
            order: None,
            processor: Raw,
        }
    }
}

impl<'a, C: Request, P: Processor + 'a> All<'a, C, P> {
    /// Execute the request and return the result
    pub fn get(self) -> impl std::future::Future<Output = Result<P::Output>> + 'a {
        Execute::get(self)
    }

    /// Convert this request to return results as a Polars DataFrame instead of raw JSON
    #[cfg(feature = "table")]
    pub fn as_dataframe(self) -> All<'a, C, crate::processor::Table> {
        All {
            client: self.client,
            ticker: self.ticker,
            ticker_type: self.ticker_type,
            market: self.market,
            exchange: self.exchange,
            limit: self.limit,
            sort: self.sort,
            order: self.order,
            processor: crate::processor::Table,
        }
    }

    /// Convert to decoded typed output (`Vec<`[`Ticker`]`>`)
    pub fn decoded(self) -> All<'a, C, Decoder<Vec<Ticker>>> {
        use crate::rest::decoded::tickers::decode;
        let decoder = Decoder::new(decode::all);

        All {
            client: self.client,
            ticker: self.ticker,
            ticker_type: self.ticker_type,
            market: self.market,
            exchange: self.exchange,
            limit: self.limit,
            sort: self.sort,
            order: self.order,
            processor: decoder,
        }
    }

    /// Set a custom decoder function to convert JSON to typed data
    pub fn with_decoder<T>(
        self,
        decoder_fn: impl Fn(decoder::Value) -> decoder::Result<T> + Send + Sync + 'static,
    ) -> All<'a, C, Decoder<T>> {
        All {
            client: self.client,
            ticker: self.ticker,
            ticker_type: self.ticker_type,
            market: self.market,
            exchange: self.exchange,
            limit: self.limit,
            sort: self.sort,
            order: self.order,
            processor: Decoder::new(decoder_fn),
        }
    }

    /// Filter by ticker symbol (e.g., "AAPL")
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    /// Filter by ticker type (e.g., "CS" for common stock)
    pub fn ticker_type(mut self, ticker_type: impl Into<String>) -> Self {
        self.ticker_type = Some(ticker_type.into());
        self
    }

    /// Filter by market (e.g., "stocks", "crypto", "fx")
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    /// Filter by primary exchange MIC (e.g., "XNAS" for NASDAQ)
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }

    /// Set the maximum number of results to return (default: 100, max: 1000)
    pub fn limit(mut self, limit: impl Into<Limit>) -> Self {
        self.limit = limit.into().into();
        self
    }

    /// Set the field to sort by
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Set the sort order (asc or desc)
    pub fn order(mut self, order: SortOrder) -> Self {
        self.order = Some(order);
        self
    }
}

impl<'a, C: Request, P: Processor + 'a> Execute for All<'a, C, P> {
    type Output = P::Output;

    #[allow(refining_impl_trait_reachable)]
    async fn get(self) -> Result<P::Output> {
        let api_key = self
            .client
            .api_key()
            .ok_or_else(|| crate::error::Error::Custom("API key not set".to_string()))?;

        let mut path = "/v3/reference/tickers".to_string();
        let mut params = vec![format!("apiKey={}", api_key)];

        if let Some(t) = self.ticker {
            params.push(format!("ticker={t}"));
        }
        if let Some(tt) = self.ticker_type {
            params.push(format!("type={tt}"));
        }
        if let Some(m) = self.market {
            params.push(format!("market={m}"));
        }
        if let Some(e) = self.exchange {
            params.push(format!("exchange={e}"));
        }
        if let Some(l) = self.limit {
            params.push(format!("limit={l}"));
        }
        if let Some(s) = self.sort {
            params.push(format!("sort={s}"));
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

/// JSON-serializable parameters for all tickers request
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct Params {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Ticker symbol to filter by (e.g., "AAPL" for Apple Inc.)
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    /// Type of ticker (e.g., "CS" for common stock, "ETF" for exchange-traded fund)
    pub ticker_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Market type (e.g., "stocks", "crypto", "fx")
    pub market: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Primary exchange MIC (Market Identifier Code) according to ISO 10383
    pub exchange: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum number of results to return (default: 100, max: 1000)
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Field to sort by
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sort order for results (asc or desc)
    pub order: Option<SortOrder>,
}
