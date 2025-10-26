//! HTTP request trait and request parameter types

use crate::error::Result;
use crate::response::Response;

use std::future::Future;

pub mod aggs;
pub mod common;
pub mod financials;
pub mod tickers;

/// Trait for HTTP clients that can make requests to the polygon.io API.
///
/// Implement this trait to use custom HTTP clients with the polygon.io client.
pub trait Request: Send + Sync {
    /// Associated response type
    type Response: Response;

    /// Create a new instance of the HTTP client
    fn new() -> Self
    where
        Self: Sized;

    /// Make an HTTP GET request to the given URL
    fn get(&self, url: &str) -> impl Future<Output = Result<Self::Response>> + Send;

    /// Make an HTTP POST request to the given URL with a JSON body
    fn post(&self, url: &str, body: &str) -> impl Future<Output = Result<Self::Response>> + Send;
}

#[cfg(feature = "reqwest")]
/// HTTP response implementation for the reqwest client
pub struct HttpResponse {
    status: u16,
    body: String,
    request_id: Option<String>,
}

#[cfg(feature = "reqwest")]
impl Response for HttpResponse {
    fn status(&self) -> u16 {
        self.status
    }

    fn body(&self) -> &str {
        &self.body
    }

    fn request_id(&self) -> &Option<String> {
        &self.request_id
    }
}

#[cfg(feature = "reqwest")]
impl Request for reqwest::Client {
    type Response = HttpResponse;

    fn new() -> Self {
        reqwest::Client::new()
    }

    async fn get(&self, url: &str) -> Result<Self::Response> {
        let response = self.get(url).send().await?;
        let status = response.status().as_u16();
        let request_id = response
            .headers()
            .get("X-Request-Id")
            .and_then(|h| h.to_str().ok().map(|s| s.to_string()));
        let body = response.text().await?;
        Ok(HttpResponse {
            status,
            body,
            request_id,
        })
    }

    async fn post(&self, url: &str, body: &str) -> Result<Self::Response> {
        let response = self.post(url).body(body.to_string()).send().await?;
        let status = response.status().as_u16();
        let request_id = response
            .headers()
            .get("X-Request-Id")
            .and_then(|h| h.to_str().ok().map(|s| s.to_string()));
        let body = response.text().await?;
        Ok(HttpResponse {
            status,
            body,
            request_id,
        })
    }
}
