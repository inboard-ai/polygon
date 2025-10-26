//! DataFrame processor. Converts JSON response to Polars DataFrame
use polars::prelude::*;
use std::io::Cursor;

use crate::error::Result;
use crate::processor::Processor;
use crate::response::Response;

/// Converts JSON response to Polars DataFrame
pub struct Table;

#[cfg(feature = "table")]
impl Processor for Table {
    type Output = polars::prelude::DataFrame;

    fn process<R: Response>(&self, response: Result<R>) -> Result<polars::prelude::DataFrame> {
        let resp = response?; // Propagate HTTP errors
        if resp.status() != 200 {
            return Err(crate::error::Error::ApiError {
                request_id: resp.request_id().to_owned(),
                status: resp.status().to_owned(),
                message: resp.body().to_owned(),
            });
        }

        let json = resp.body();
        let json_value: serde_json::Value = serde_json::from_str(json)?;
        let results = json_value
            .get("results")
            .ok_or_else(|| crate::error::Error::Custom("Missing 'results' field".into()))?;

        let json_bytes = serde_json::to_vec(results)?;
        let df = JsonReader::new(Cursor::new(json_bytes))
            .finish()
            .map_err(|e| crate::error::Error::Custom(format!("Polars error: {e}")))?;
        Ok(df)
    }
}
