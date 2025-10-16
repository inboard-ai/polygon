//! HTTP request trait and implementations

use crate::error::Result;

use std::future::Future;

/// Trait for HTTP response objects
pub trait Response {
    /// Get the HTTP status code
    fn status(&self) -> u16;

    /// Get the response body as a string
    fn body(self) -> String;
}

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
pub struct HttpResponse {
    status: u16,
    body: String,
}

#[cfg(feature = "reqwest")]
impl Response for HttpResponse {
    fn status(&self) -> u16 {
        self.status
    }

    fn body(self) -> String {
        self.body
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
        let body = response.text().await?;
        Ok(HttpResponse { status, body })
    }

    async fn post(&self, url: &str, body: &str) -> Result<Self::Response> {
        let response = self.post(url).body(body.to_string()).send().await?;
        let status = response.status().as_u16();
        let body = response.text().await?;
        Ok(HttpResponse { status, body })
    }
}
