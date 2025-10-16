//! HTTP request trait and implementations

use crate::error::Result;

use std::future::Future;

/// Trait for HTTP clients that can make requests to the polygon.io API.
///
/// Implement this trait to use custom HTTP clients with the polygon.io client.
pub trait Request: Send + Sync {
    /// Create a new instance of the HTTP client
    fn new() -> Self
    where
        Self: Sized;

    /// Make an HTTP GET request to the given URL
    fn get(&self, url: &str) -> impl Future<Output = Result<String>> + Send;

    /// Make an HTTP POST request to the given URL with a JSON body
    fn post(&self, url: &str, body: &str) -> impl Future<Output = Result<String>> + Send;
}

#[cfg(feature = "reqwest")]
impl Request for reqwest::Client {
    fn new() -> Self {
        reqwest::Client::new()
    }

    async fn get(&self, url: &str) -> Result<String> {
        let response = self.get(url).send().await?;
        let text = response.text().await?;
        Ok(text)
    }

    async fn post(&self, url: &str, body: &str) -> Result<String> {
        let response = self
            .post(url)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?;
        let text = response.text().await?;
        Ok(text)
    }
}
