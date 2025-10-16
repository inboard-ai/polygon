# polygon

Rust async client library for [polygon.io](https://polygon.io) API.

## Features

- **`reqwest`** (enabled by default): Uses `reqwest` as the default HTTP client with a default type parameter
- **`dotenvy`** (enabled by default): Enables loading API keys from environment variables via `.env` files

## Usage

### With both `reqwest` and `dotenvy` (default)

```rust
use polygon::{Polygon, rest::quotes};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Loads POLYGON_API_KEY from environment
    let client = Polygon::new()?;
    
    // Call API endpoints using standalone functions
    let quote = quotes::get_last_quote(&client, "AAPL").await?;
    println!("{}", quote);
    
    Ok(())
}
```

### With `reqwest` only (manual API key)

```rust
use polygon::{Polygon, rest::quotes};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Manually provide API key
    let client = Polygon::default().with_key("your_api_key");
    
    let quote = quotes::get_last_quote(&client, "AAPL").await?;
    println!("{}", quote);
    
    Ok(())
}
```

### With custom HTTP client (no `reqwest`)

```rust
use polygon::{Polygon, Request, rest::quotes};

// Implement the Request trait for your custom client
struct MyHttpClient;

impl Request for MyHttpClient {
    fn new() -> Self {
        MyHttpClient
    }
    
    fn get(&self, url: &str) -> impl Future<Output = Result<String>> + Send {
        async move {
            // Your custom HTTP implementation
            Ok(String::new())
        }
    }
    
    fn post(&self, url: &str, body: &str) -> impl Future<Output = Result<String>> + Send {
        async move {
            // Your custom HTTP implementation
            Ok(String::new())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http_client = MyHttpClient::new();
    
    // With dotenvy enabled, this loads the API key from env
    let client = Polygon::with_client(http_client)?;
    
    // Or without dotenvy:
    // let client = Polygon::with_client(http_client).with_key("your_api_key");
    
    let quote = quotes::get_last_quote(&client, "AAPL").await?;
    Ok(())
}
```

## API Structure

The library is organized into modules:

- `client`: The main `Polygon` client struct
- `error`: Error types and `Result` alias
- `request`: The `Request` trait for HTTP clients
- `rest`: REST API endpoints
  - `rest::quotes`: Quote-related functions

## Feature Flags

### Default features

```toml
[dependencies]
polygon = "0.1"
```

### Only `reqwest` (no environment variable loading)

```toml
[dependencies]
polygon = { version = "0.1", default-features = false, features = ["reqwest"] }
```

### Only `dotenvy` (bring your own HTTP client)

```toml
[dependencies]
polygon = { version = "0.1", default-features = false, features = ["dotenvy"] }
```

### No default features (full customization)

```toml
[dependencies]
polygon = { version = "0.1", default-features = false }
```

## Environment Variables

When the `dotenvy` feature is enabled, the library will attempt to load environment variables from a `.env` file in the current directory:

```
POLYGON_API_KEY=your_api_key_here
```

## License

MIT
