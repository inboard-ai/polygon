<div align="center">

# polygon

[![Crates.io](https://img.shields.io/crates/v/polygon.svg)](https://crates.io/crates/polygon)
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

## Available Endpoints

- `rest::aggs` - Aggregate bars (OHLC data)
- `rest::quotes` - Real-time and historical quotes

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
