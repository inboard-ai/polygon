//! Query parameter builder using the decoder crate

use crate::client::Polygon;
use crate::error::{Error, Result};
use crate::request::{Request, Response};
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
pub struct Query<'a, Client: Request, T = ()> {
    client: &'a Polygon<Client>,
    base_url: String,
    params: Vec<(&'static str, Value)>,
    required: HashSet<&'static str>,
    allowed: HashSet<&'static str>,
    #[cfg(feature = "decoder")]
    decoder: Option<Box<dyn Fn(decoder::Value) -> decoder::Result<T> + 'a>>,
    #[cfg(not(feature = "decoder"))]
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, Client: Request, T> Query<'a, Client, T> {
    /// Add a parameter to the query
    pub fn param<V: Param>(mut self, key: &'static str, value: V) -> Self {
        self.params.push((key, value.encode()));
        self
    }

    /// Add multiple parameters to the query
    pub fn params<V: Param>(mut self, params: impl IntoIterator<Item = (&'static str, V)>) -> Self {
        self.params
            .extend(params.into_iter().map(|(k, v)| (k, v.encode())));
        self
    }

    fn execute_request(mut self) -> impl std::future::Future<Output = Result<String>> {
        async move {
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

            // Validate allowed params (if any are specified)
            if !self.allowed.is_empty() {
                let invalid: Vec<_> = provided
                    .difference(&self.allowed)
                    .filter(|k| **k != "apiKey")
                    .copied()
                    .collect();
                if !invalid.is_empty() {
                    let allowed_list: Vec<_> = self.allowed.iter().copied().collect();
                    return Err(Error::Custom(format!(
                        "Invalid parameters: {:?}. Allowed: {:?}",
                        invalid, allowed_list
                    )));
                }
            }

            // Build query string
            let map = encode::map(self.params);
            let query_string = serde_urlencoded::to_string(&Value::from(map))
                .map_err(|e| Error::Custom(e.to_string()))?;

            // Execute request
            let url = format!("{}?{}", self.base_url, query_string);
            let response = self.client.client().get(&url).await?;

            // Check status and extract body
            let status = response.status();
            let body = response.body();

            if status >= 400 {
                // Handle duplicate JSON from polygon.io server bug (???)
                let json_to_parse = if let Some(dup_pos) = body.find("}{") {
                    &body[..dup_pos + 1]
                } else {
                    &body
                };

                if let Ok(value) = serde_json::from_str::<serde_json::Value>(json_to_parse) {
                    let message = value
                        .get("error")
                        .or_else(|| value.get("message"))
                        .and_then(|e| e.as_str())
                        .unwrap_or("Unknown error")
                        .to_string();

                    let request_id = value
                        .get("request_id")
                        .and_then(|id| id.as_str())
                        .map(|s| s.to_string());

                    return Err(Error::ApiError {
                        status,
                        message,
                        request_id,
                    });
                }
                // Fallback if response isn't JSON
                let error_msg = format!("HTTP {}: {}", status, body);
                return Err(Error::Custom(error_msg));
            }

            Ok(body)
        }
    }
}

impl<'a, Client: Request> Query<'a, Client, ()> {
    /// Create a new query builder (internal use)
    pub(crate) fn new(client: &'a Polygon<Client>, base_url: impl Into<String>) -> Self {
        Query {
            client,
            base_url: base_url.into(),
            params: Vec::new(),
            required: HashSet::new(),
            allowed: HashSet::new(),
            #[cfg(feature = "decoder")]
            decoder: None,
            #[cfg(not(feature = "decoder"))]
            _phantom: std::marker::PhantomData,
        }
    }

    /// Mark a parameter as required (internal use)
    #[allow(dead_code)]
    pub(crate) fn require(mut self, key: &'static str) -> Self {
        self.required.insert(key);
        self.allowed.insert(key);
        self
    }

    /// Mark a parameter as optional (internal use)
    pub(crate) fn optional(mut self, key: &'static str) -> Self {
        self.allowed.insert(key);
        self
    }

    #[cfg(feature = "decoder")]
    /// Set the decoder function for typed responses
    pub fn with_decoder<U>(
        self,
        decoder_fn: impl Fn(decoder::Value) -> decoder::Result<U> + 'a,
    ) -> Query<'a, Client, U> {
        Query {
            client: self.client,
            base_url: self.base_url,
            params: self.params,
            required: self.required,
            allowed: self.allowed,
            decoder: Some(Box::new(decoder_fn)),
        }
    }
}

#[cfg(feature = "decoder")]
/// Marker trait for types that can be decoded from API responses
pub trait Decodable {}

/// Trait for executing queries
pub trait Execute {
    /// The output type of the query execution
    type Output;

    /// Execute the query and return the result
    fn get(self) -> impl std::future::Future<Output = Result<Self::Output>>;
}

impl<'a, Client: Request> Execute for Query<'a, Client, ()> {
    type Output = String;
    fn get(self) -> impl std::future::Future<Output = Result<String>> {
        self.execute_request()
    }
}

#[cfg(feature = "decoder")]
impl<'a, Client: Request, T: Decodable> Execute for Query<'a, Client, T> {
    type Output = T;
    fn get(self) -> impl std::future::Future<Output = Result<T>> {
        let Query {
            client,
            base_url,
            params,
            required,
            allowed,
            decoder,
            ..
        } = self;
        async move {
            let decoder = decoder.ok_or_else(|| Error::Custom("No decoder set".to_string()))?;
            let query = Query {
                client,
                base_url,
                params,
                required,
                allowed,
                decoder: None::<Box<dyn Fn(decoder::Value) -> decoder::Result<()>>>,
                #[cfg(not(feature = "decoder"))]
                _phantom: std::marker::PhantomData,
            };
            let json = query.execute_request().await?;
            Ok(decoder::run(serde_json::from_str, decoder, &json)?)
        }
    }
}
