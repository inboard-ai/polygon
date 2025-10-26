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
    let client = Polygon::default().with_key("your_api_key");
    let json = aggs::previous_close(&client, "AAPL").get().await?;
    println!("{}", json);
    Ok(())
}
```

Set your API key via environment variable:
```bash
export POLYGON_API_KEY=your_key_here
```

Or use a `.env` file with the `dotenvy` feature:
```rust
let client = Polygon::new()?; // Loads from POLYGON_API_KEY env var
```

## API Design

Each endpoint returns a request builder. Call `.get()` to execute:

```rust
use polygon::rest::tickers;

// Raw JSON response
let json = tickers::related(&client, "AAPL").get().await?;

// Decoded into typed structs
let data = tickers::all(&client)
    .limit(10)
    .exchange("XNYS")
    .decoded() // decode it into types
    .get()
    .await?;

println!("{:?} {:?}", data[0].ticker, data[0].name);
```

### Three Access Modes

**Raw JSON** (`rest::raw::*`, exported to `rest::*` for convenience) returns raw JSON strings:
```rust
let json = rest::aggs::aggregates(&client, "AAPL", 1, Timespan::Day, "2024-01-01", "2024-01-31")
    .adjusted(true)
    .limit(5)
    .get()
    .await?;
```

**Decoded** (`rest::decoded::*`) returns typed Rust structs (requires `decoder` feature):
```rust
let aggs: Vec<Agg> = decoded::aggs::aggregates(&client, "AAPL", 1, Timespan::Day, "2024-01-01", "2024-01-31")
    .get()
    .await?;
```

**Table** (`rest::table::*`) returns `polars` DataFrames (requires `table` feature):
```rust
let df: DataFrame = table::aggs::aggregates(&client, "AAPL", 1, Timespan::Day, "2024-01-01", "2024-01-31")
    .get()
    .await?;
```

### Features

- **`reqwest`** (default): Uses [`reqwest`](https://docs.rs/reqwest) as the HTTP client. Disable to provide your own client.
- **`decoder`** (default): Enables typed response decoding via [`decoder`](https://docs.rs/decoder). Provides `rest::decoded::*` modules.
- **`table`**: Enables Polars DataFrame support via [`polars`](https://docs.rs/polars). Provides `rest::table::*` modules.
- **`dotenvy`**: Enables loading API keys from `.env` files. Adds `Polygon::new()` constructor.

## Available Endpoints

**Aggregates (OHLCV bars)**
- `aggregates()` - Get bars over a date range
- `previous_close()` - Get previous day's OHLC
- `grouped_daily()` - Get daily bars for entire market
- `daily_open_close()` - Get open/close for specific date

**Tickers (Reference data)**
- `all()` - List all tickers with filters
- `details()` - Get detailed ticker information
- `related()` - Get related companies
- `types()` - Get all ticker types
- `events()` - Get corporate events
- `news()` - Get recent news

**Financials (Company financials)**
- `balance_sheets()` - Balance sheet data
- `cash_flow_statements()` - Cash flow statements
- `income_statements()` - Income statements
- `ratios()` - Financial ratios

## LLM Tool Use

The library includes a progressive discovery interface for LLMs to explore and call endpoints dynamically:

```rust
use polygon::tool_use;
use serde_json::json;

// List available tools
let tools = tool_use::list_tools();

// Discover API structure
let modules = tool_use::call_tool(&client, json!({
    "tool": "list_modules",
    "params": {}
})).await?;

// Get endpoint schema
let schema = tool_use::call_tool(&client, json!({
    "tool": "get_endpoint_schema",
    "params": {
        "module": "Aggs",
        "endpoint": "aggregates"
    }
})).await?;

// Call endpoint with parameters
let data = tool_use::call_tool(&client, json!({
    "tool": "call_endpoint",
    "params": {
        "module": "Aggs",
        "endpoint": "aggregates",
        "arguments": {
            "ticker": "AAPL",
            "multiplier": 1,
            "timespan": "day",
            "from": "2024-01-01",
            "to": "2024-01-31"
        }
    }
})).await?;
```

See `examples/llm_exploration.rs` for a complete walkthrough.

## Custom HTTP Client

Implement the `Request` trait to use your own HTTP client:

```rust
use polygon::{Request, Polygon, Result};

struct MyClient;

impl Request for MyClient {
    async fn get(&self, url: &str) -> Result<String> {
        // Your implementation
        Ok(String::new())
    }
}

let client = Polygon::with_client(MyClient, Some("api_key".to_string()));
```

## License

MIT
