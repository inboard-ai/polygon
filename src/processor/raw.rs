//! Raw JSON processor. Returns raw JSON string from response body
use crate::error::Result;
use crate::processor::Processor;
use crate::response::Response;

/// Raw JSON string processor - returns the HTTP response body as a String without any transformation
pub struct Raw;

impl Processor for Raw {
    type Output = String;

    fn process<R: Response>(&self, response: Result<R>) -> Result<String> {
        let resp = response?; // Propagate HTTP errors
        if resp.status() != 200 {
            return Err(crate::error::Error::ApiError {
                request_id: resp.request_id().to_owned(),
                status: resp.status().to_owned(),
                message: resp.body().to_owned(),
            });
        }
        Ok(resp.body().to_owned())
    }
}
