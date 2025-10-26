use crate::client::Polygon;
use crate::error::Result;
use crate::execute::Execute;
use crate::processor::{Processor, Raw};
use crate::request::Request;

/// Request builder for ticker types
pub struct Types<'a, Client: Request, P: Processor = Raw> {
    client: &'a Polygon<Client>,
    /// Asset class to filter by
    pub asset_class: Option<String>,
    /// Locale to filter by
    pub locale: Option<String>,
    processor: P,
}

impl<'a, C: Request> Types<'a, C, Raw> {
    /// Create a new ticker types request
    pub fn new(client: &'a Polygon<C>) -> Self {
        Self {
            client,
            asset_class: None,
            locale: None,
            processor: Raw,
        }
    }
}

impl<'a, C: Request, P: Processor + 'a> Types<'a, C, P> {
    /// Execute the request and return the result
    pub fn get(self) -> impl std::future::Future<Output = Result<P::Output>> + 'a {
        Execute::get(self)
    }

    /// Convert this request to return results as a Polars DataFrame instead of raw JSON
    #[cfg(feature = "table")]
    pub fn as_dataframe(self) -> Types<'a, C, crate::processor::Table> {
        Types {
            client: self.client,
            asset_class: self.asset_class,
            locale: self.locale,
            processor: crate::processor::Table,
        }
    }

    /// Filter by asset class
    pub fn asset_class(mut self, ac: impl Into<String>) -> Self {
        self.asset_class = Some(ac.into());
        self
    }

    /// Filter by locale
    pub fn locale(mut self, l: impl Into<String>) -> Self {
        self.locale = Some(l.into());
        self
    }
}

impl<'a, C: Request, P: Processor + 'a> Execute for Types<'a, C, P> {
    type Output = P::Output;

    async fn get(self) -> Result<P::Output> {
        let api_key = self
            .client
            .api_key()
            .ok_or_else(|| crate::error::Error::Custom("API key not set".to_string()))?;

        let mut path = "/v3/reference/tickers/types".to_string();
        let mut params = vec![format!("apiKey={}", api_key)];

        if let Some(ac) = self.asset_class {
            params.push(format!("asset_class={ac}"));
        }
        if let Some(l) = self.locale {
            params.push(format!("locale={l}"));
        }

        path.push('?');
        path.push_str(&params.join("&"));
        let url = format!("https://api.polygon.io{path}");
        let response = self.client.client().get(&url).await;
        self.processor.process(response)
    }
}
