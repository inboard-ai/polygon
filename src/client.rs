//! Main polygon.io API client
use crate::request::Request;

/// The main polygon.io API client.
///
/// When the `reqwest` feature is enabled, this uses `reqwest::Client` as the default HTTP client.
/// When the `reqwest` feature is disabled, you must provide your own HTTP client that implements [`Request`].
#[cfg(feature = "reqwest")]
#[derive(Debug, Clone)]
pub struct Polygon<Client: Request = reqwest::Client> {
    client: Client,
    api_key: Option<String>,
}

/// The main polygon.io API client.
///
/// When the `reqwest` feature is enabled, this uses `reqwest::Client` as the default HTTP client.
/// When the `reqwest` feature is disabled, you must provide your own HTTP client that implements [`Request`].
#[cfg(not(feature = "reqwest"))]
#[derive(Debug, Clone)]
pub struct Polygon<Client: Request> {
    client: Client,
    api_key: Option<String>,
}

// Implementation for any Client that implements Request
impl<Client: Request> Polygon<Client> {
    /// Create a new polygon.io client using the default HTTP client.
    ///
    /// This method is only available when the `dotenvy` feature is enabled.
    /// It loads the API key from the `POLYGON_API_KEY` environment variable using dotenvy.
    ///
    /// # Errors
    ///
    /// Returns an error if the environment variable cannot be loaded or if the API key is missing.
    #[cfg(feature = "dotenvy")]
    pub fn new() -> crate::Result<Self> {
        dotenvy::dotenv().ok(); // Try to load .env file, ignore errors

        let api_key = std::env::var("POLYGON_API_KEY").map_err(|_| crate::Error::MissingApiKey)?;

        Ok(Self {
            client: Client::new(),
            api_key: Some(api_key),
        })
    }
    /// Create a new polygon.io client with a custom HTTP client.
    ///
    /// This method is available regardless of which features are enabled.
    /// When the `dotenvy` feature is enabled, it will attempt to load the API key
    /// from the `POLYGON_API_KEY` environment variable.
    ///
    /// # Errors
    ///
    /// Returns an error if the `dotenvy` feature is enabled and the API key cannot be loaded.
    #[cfg(feature = "dotenvy")]
    pub fn with_client(client: Client) -> crate::Result<Self> {
        dotenvy::dotenv().ok(); // Try to load .env file, ignore errors

        let api_key = std::env::var("POLYGON_API_KEY").map_err(|_| crate::Error::MissingApiKey)?;

        Ok(Self {
            client,
            api_key: Some(api_key),
        })
    }

    /// Create a new polygon.io client with a custom HTTP client.
    ///
    /// This method is available when the `dotenvy` feature is disabled.
    /// You must manually set the API key using [`with_key`](Self::with_key).
    #[cfg(not(feature = "dotenvy"))]
    pub fn with_client(client: Client) -> Self {
        Self { client, api_key: None }
    }

    /// Set the API key for this client.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use polygon::Polygon;
    ///
    /// let client = Polygon::default().with_key("my_api_key");
    /// ```
    pub fn with_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Get the API key for this client.
    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    /// Get a reference to the underlying HTTP client.
    pub fn client(&self) -> &Client {
        &self.client
    }
}

// Default implementation
#[cfg(feature = "reqwest")]
impl Default for Polygon<reqwest::Client> {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: None,
        }
    }
}

// Default implementation
#[cfg(not(feature = "reqwest"))]
impl<Client: Request> Default for Polygon<Client> {
    /// Create a default polygon.io client with no API key set.
    ///
    /// You must call [`with_key`](Self::with_key) to set the API key before making requests.
    fn default() -> Self {
        Self {
            client: Client::new(),
            api_key: None,
        }
    }
}
