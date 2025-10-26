//! Decoder processor for converting JSON to typed data
use std::sync::Arc;

use crate::processor::Processor;

/// Decoder processor marker type - holds a decoder function to convert JSON to typed data
///
/// The decoder function is stored in this processor and used by request builders
/// when executing requests that need typed output.
pub struct Decoder<T> {
    pub(crate) decoder_fn: Arc<dyn Fn(decoder::Value) -> decoder::Result<T> + Send + Sync>,
}

impl<T> Decoder<T> {
    /// Create a new Decoder with the given decoding function
    pub fn new(decoder_fn: impl Fn(decoder::Value) -> decoder::Result<T> + Send + Sync + 'static) -> Self {
        Self {
            decoder_fn: Arc::new(decoder_fn),
        }
    }
}

impl<T> Processor for Decoder<T> {
    type Output = T;

    fn process<R: crate::response::Response>(&self, response: crate::error::Result<R>) -> crate::error::Result<T> {
        // Get raw response
        let resp = response?;
        if resp.status() != 200 {
            return Err(crate::error::Error::ApiError {
                request_id: resp.request_id().to_owned(),
                status: resp.status().to_owned(),
                message: resp.body().to_owned(),
            });
        }

        let json = resp.body();

        // Use decoder::run to parse JSON and decode in one step
        decoder::run(serde_json::from_str, &*self.decoder_fn, json)
            .map_err(|e| crate::error::Error::Custom(format!("Decode error: {e}")))
    }
}
