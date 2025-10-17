<div align="center">

# polygon

[![Crates.io](https://img.shields.io/crates/v/polygon.svg)](https://crates.io/crates/polygon)
[![Documentation](https://docs.rs/polygon/badge.svg)](https://docs.rs/polygon)
[![License](https://img.shields.io/crates/l/polygon.svg)](https://github.com/inboard-ai/polygon/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/polygon.svg)](https://crates.io/crates/polygon)

A Rust async client library for [polygon.io](https://polygon.io)

</div>

## Quick Start

```toml
[dependencies]
polygon = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

```rust
use polygon::Polygon;
use polygon::rest::aggs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Polygon::new()?;
    let result = aggs::get_previous_close(&client, "AAPL").await?;
    println!("{}", result);
    Ok(())
}
```

Set your API key via environment variable:
```bash
export POLYGON_API_KEY=your_key_here
```

Or use a `.env` file, or set it manually:
```rust
let client = Polygon::default().with_key("your_api_key");
```

## Query API

Endpoints return a `Query` builder. Call `.get()` to execute:

```rust
use polygon::query::Execute as _;
use polygon::rest::{raw, tickers};

// Raw JSON response
let json = raw::tickers::related(&client, "AAPL").get().await?;

// Decoded into typed structs
let data = tickers::all(&client)
    .param("limit", 10)
    .params([("exchange", "XNYS"), ("sort", "ticker")])
    .get()
    .await?;

println!("{} {}", data[0].ticker, data[0].name);
```

**Design:**
- Required parameters are function arguments, optional parameters use `.param()` or `.params()`
- `rest::raw::foo` returns JSON strings, `rest::foo` returns decoded types
- Same API for both: construct query, chain parameters, call `.get()`
- `.param()` accepts any type implementing `Param` (integers, strings, bools)
- Errors include HTTP status, message, and request ID

## Available Endpoints

- `rest::aggs` - Aggregate bars (OHLC)
- `rest::tickers` - Ticker reference data

## Custom HTTP Client

Implement the `Request` trait to use your own HTTP client:

```rust
use polygon::{Request, Polygon};

struct MyClient;

impl Request for MyClient {
    fn new() -> Self { MyClient }
    fn get(&self, url: &str) -> impl Future<Output = Result<String>> + Send {
        async move { /* your implementation */ Ok(String::new()) }
    }
    fn post(&self, url: &str, body: &str) -> impl Future<Output = Result<String>> + Send {
        async move { /* your implementation */ Ok(String::new()) }
    }
}

let client = Polygon::with_client(MyClient)?;
```

## License

MIT
