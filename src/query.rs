//! Query parameter builder using the decoder crate

use crate::client::Polygon;
use crate::error::{Error, Result};
use crate::request::Request;
use decoder::Value;
use std::collections::HashSet;

pub use decoder::encode;

/// Trait for types that can be converted into query parameter values
pub trait Param {
    /// Convert the parameter into a query parameter value
    fn encode(self) -> Value;
}

impl Param for &str {
    fn encode(self) -> Value {
        encode::string(self)
    }
}

impl Param for String {
    fn encode(self) -> Value {
        encode::string(self)
    }
}

impl Param for bool {
    fn encode(self) -> Value {
        encode::bool(self)
    }
}

impl Param for u32 {
    fn encode(self) -> Value {
        encode::u32(self)
    }
}

impl Param for i32 {
    fn encode(self) -> Value {
        encode::i32(self)
    }
}

impl Param for u64 {
    fn encode(self) -> Value {
        encode::u64(self)
    }
}

impl Param for i64 {
    fn encode(self) -> Value {
        encode::i64(self)
    }
}

impl Param for f32 {
    fn encode(self) -> Value {
        encode::f32(self)
    }
}

impl Param for f64 {
    fn encode(self) -> Value {
        encode::f64(self)
    }
}

impl Param for Value {
    fn encode(self) -> Value {
        self
    }
}

impl<T: Param> Param for Option<T> {
    fn encode(self) -> Value {
        encode::optional(|v| v.encode(), self)
    }
}

/// Query parameter builder with request execution
pub struct Query<'a, Client: Request> {
    client: &'a Polygon<Client>,
    base_url: String,
    params: Vec<(&'static str, Value)>,
    required: HashSet<&'static str>,
}

impl<'a, Client: Request> Query<'a, Client> {
    /// Create a new query builder (internal use)
    pub(crate) fn new(client: &'a Polygon<Client>, base_url: impl Into<String>) -> Self {
        Query {
            client,
            base_url: base_url.into(),
            params: Vec::new(),
            required: HashSet::new(),
        }
    }

    /// Mark a parameter as required (internal use)
    pub(crate) fn require(mut self, key: &'static str) -> Self {
        self.required.insert(key);
        self
    }

    /// Add a parameter to the query
    pub fn param<V: Param>(mut self, key: &'static str, value: V) -> Self {
        self.params.push((key, value.encode()));
        self
    }

    /// Execute the request
    pub async fn get(mut self) -> Result<String> {
        // Add API key
        let api_key = self.client.api_key().ok_or(Error::MissingApiKey)?;
        self.params.push(("apiKey", encode::string(api_key)));

        // Validate required params
        let provided: HashSet<_> = self.params.iter().map(|(k, _)| *k).collect();
        let missing: Vec<_> = self.required.difference(&provided).copied().collect();
        if !missing.is_empty() {
            return Err(Error::Custom(format!(
                "Missing required parameters: {:?}",
                missing
            )));
        }

        // Build query string
        let map = encode::map(self.params);
        let query_string = serde_urlencoded::to_string(&Value::from(map))
            .map_err(|e| Error::Custom(e.to_string()))?;

        // Execute request
        let url = format!("{}?{}", self.base_url, query_string);
        self.client.client().get(&url).await
    }
}


