//! Response processors for converting HTTP responses to desired output types

use crate::error::Result;
use crate::response::Response;

pub mod raw;

pub use raw::Raw;

#[cfg(feature = "table")]
pub mod table;

#[cfg(feature = "table")]
pub use table::Table;

pub mod decoder;

pub use decoder::Decoder;

/// Converts an HTTP response to the desired output type
pub trait Processor {
    /// The output type this processor produces
    type Output;

    /// Process the response (or error)
    ///
    /// The processor handles both success and failure cases.
    /// It can inspect the response, validate status codes, and convert the body.
    fn process<R: Response>(&self, response: Result<R>) -> Result<Self::Output>;
}
