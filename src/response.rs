//! Response types for polygon.io API
//!
//! This module contains all response/output types returned by the API.

pub mod aggs;
pub mod financials;
pub mod ticker;

/// Trait for HTTP response objects
pub trait Response {
    /// Get the HTTP status code
    fn status(&self) -> u16;

    /// Get the response body as a string
    fn body(&self) -> &str;

    /// The ID of the corresponding request
    fn request_id(&self) -> &Option<String>;
}
