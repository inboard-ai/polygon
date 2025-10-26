//! LLM Tool Use Interface for Polygon API
//!
//! This module provides a progressive discovery interface that allows LLMs (and other automated
//! systems) to explore and interact with the Polygon.io API without prior knowledge of its structure.
//!
//! # Available Tools
//!
//! | Tool | Purpose |
//! |------|---------|
//! | `list_tools` | Get catalog of all available tools |
//! | `list_modules` | Get all API modules (Tickers, Aggs, Financials) |
//! | `list_endpoints` | Get all endpoints within a module |
//! | `get_endpoint_schema` | Get JSON Schema for endpoint parameters |
//! | `call_endpoint` | Execute an API call with parameters |
//!
//! All tools are invoked through [`call_tool()`].
//!
//! # Example: Full Discovery Flow
//!
//! ```no_run
//! use polygon::Polygon;
//! use polygon::tool_use;
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Polygon::default().with_key("your_api_key");
//!
//! // Step 1: What can I do?
//! let tools = tool_use::call_tool(&client, json!({
//!     "tool": "list_tools",
//!     "params": {}
//! })).await?;
//! println!("Available tools: {}", tools);
//!
//! // Step 2: What modules exist?
//! let modules = tool_use::call_tool(&client, json!({
//!     "tool": "list_modules",
//!     "params": {}
//! })).await?;
//! // Returns: ["Tickers", "Aggs", "Financials"]
//!
//! // Step 3: What endpoints are in the Aggs module?
//! let endpoints = tool_use::call_tool(&client, json!({
//!     "tool": "list_endpoints",
//!     "params": { "module": "Aggs" }
//! })).await?;
//! // Returns: ["aggregates", "previous_close", "grouped_daily", ...]
//!
//! // Step 4: What parameters does 'aggregates' need?
//! let schema = tool_use::call_tool(&client, json!({
//!     "tool": "get_endpoint_schema",
//!     "params": {
//!         "module": "Aggs",
//!         "endpoint": "aggregates"
//!     }
//! })).await?;
//! // Returns: JSON Schema with parameter types, descriptions, requirements
//!
//! // Step 5: Call the endpoint with parameters
//! let data = tool_use::call_tool(&client, json!({
//!     "tool": "call_endpoint",
//!     "params": {
//!         "module": "Aggs",
//!         "endpoint": "aggregates",
//!         "arguments": {
//!             "ticker": "AAPL",
//!             "multiplier": 1,
//!             "timespan": "week",
//!             "from": "2024-01-01",
//!             "to": "2024-01-31"
//!         }
//!     }
//! })).await?;
//! // Returns: Actual JSON response from Polygon.io
//! # Ok(())
//! # }
//! ```
//!
//! See `examples/llm_exploration.rs` for a complete working example.

use schemars::schema_for;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::client::Polygon;
use crate::endpoint::{Aggs, Endpoint, Financials, Tickers};
use crate::error::{Error, Result};
use crate::request::Request;
use crate::request::{aggs, financials, tickers};

/// Tool metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Tool name
    pub name: String,
    /// Tool description
    pub description: String,
    /// JSON Schema for parameters
    pub parameters: Value,
}

/// List all available tools
fn list_tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "list_modules".to_string(),
            description: "List all API modules (categories of endpoints)".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {}
            }),
        },
        Tool {
            name: "list_endpoints".to_string(),
            description: "List all endpoints within a module".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "module": {
                        "type": "string",
                        "description": "Module name (e.g., 'Tickers', 'Aggs', 'Financials')",
                        "enum": ["Tickers", "Aggs", "Financials"]
                    }
                },
                "required": ["module"]
            }),
        },
        Tool {
            name: "get_endpoint_schema".to_string(),
            description: "Get the parameter schema for a specific endpoint".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "module": {
                        "type": "string",
                        "description": "Module name",
                        "enum": ["Tickers", "Aggs", "Financials"]
                    },
                    "endpoint": {
                        "type": "string",
                        "description": "Endpoint name (e.g., 'aggregates', 'details')"
                    }
                },
                "required": ["module", "endpoint"]
            }),
        },
        Tool {
            name: "call_endpoint".to_string(),
            description: "Call an API endpoint to get actual data".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "module": {
                        "type": "string",
                        "description": "Module name",
                        "enum": ["Tickers", "Aggs", "Financials"]
                    },
                    "endpoint": {
                        "type": "string",
                        "description": "Endpoint name"
                    },
                    "arguments": {
                        "type": "object",
                        "description": "Endpoint-specific arguments (use get_endpoint_schema to discover)"
                    }
                },
                "required": ["module", "endpoint", "arguments"]
            }),
        },
    ]
}

/// Universal tool caller
pub async fn call_tool<Client: Request>(client: &Polygon<Client>, request: Value) -> Result<Value> {
    let tool = request
        .get("tool")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::Custom("Missing 'tool' field".to_string()))?;

    let params = request
        .get("params")
        .ok_or_else(|| Error::Custom("Missing 'params' field".to_string()))?;

    match tool {
        "list_tools" => Ok(serde_json::to_value(list_tools())?),
        "list_modules" => list_modules(),
        "list_endpoints" => list_endpoints(params),
        "get_endpoint_schema" => get_endpoint_schema(params),
        "call_endpoint" => call_endpoint(client, params).await,
        _ => Err(Error::Custom(format!("Unknown tool: {tool}"))),
    }
}

/// List all modules
fn list_modules() -> Result<Value> {
    Ok(json!({
        "modules": [
            {
                "name": "Tickers",
                "description": "Ticker symbols, company details, news, events"
            },
            {
                "name": "Aggs",
                "description": "OHLCV aggregate data, historical prices"
            },
            {
                "name": "Financials",
                "description": "Financial statements, balance sheets, ratios"
            }
        ]
    }))
}

/// List endpoints in a module
fn list_endpoints(params: &Value) -> Result<Value> {
    let module = params
        .get("module")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::Custom("Missing 'module' parameter".to_string()))?;

    let endpoints = match module {
        "Tickers" => vec![
            json!({"name": "all", "description": "List all tickers with filters"}),
            json!({"name": "details", "description": "Get detailed info about a ticker"}),
            json!({"name": "related", "description": "Get related companies"}),
            json!({"name": "types", "description": "Get all ticker types"}),
            json!({"name": "events", "description": "Get corporate events"}),
            json!({"name": "news", "description": "Get recent news"}),
        ],
        "Aggs" => vec![
            json!({"name": "aggregates", "description": "Get OHLCV bars over date range"}),
            json!({"name": "previous_close", "description": "Get previous day close"}),
            json!({"name": "grouped_daily", "description": "Get daily bars for entire market"}),
            json!({"name": "daily_open_close", "description": "Get open/close for specific date"}),
        ],
        "Financials" => vec![
            json!({"name": "balance_sheets", "description": "Get balance sheet data"}),
            json!({"name": "cash_flow_statements", "description": "Get cash flow statements"}),
            json!({"name": "income_statements", "description": "Get income statements"}),
            json!({"name": "ratios", "description": "Get financial ratios"}),
        ],
        _ => return Err(Error::Custom(format!("Unknown module: {module}"))),
    };

    Ok(json!({"endpoints": endpoints}))
}

/// Get schema for a specific endpoint
fn get_endpoint_schema(params: &Value) -> Result<Value> {
    let module = params
        .get("module")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::Custom("Missing 'module' parameter".to_string()))?;

    let endpoint = params
        .get("endpoint")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::Custom("Missing 'endpoint' parameter".to_string()))?;

    let no_params = || {
        json!({
            "type": "object",
            "properties": {},
            "description": "No parameters required"
        })
    };

    // Generate schema for the specific endpoint type
    let schema = match (module, endpoint) {
        ("Aggs", "aggregates") => schema_for!(aggs::aggregates::Params),
        ("Aggs", "previous_close") => schema_for!(aggs::previous_close::Params),
        ("Aggs", "grouped_daily") => schema_for!(aggs::grouped_daily::Params),
        ("Aggs", "daily_open_close") => schema_for!(aggs::daily_open_close::Params),
        ("Tickers", "all") => schema_for!(tickers::all::Params),
        ("Tickers", "details") => schema_for!(tickers::details::Params),
        ("Tickers", "related") => schema_for!(tickers::related::Params),
        ("Tickers", "events") => schema_for!(tickers::events::Params),
        ("Tickers", "news") => schema_for!(tickers::news::Params),
        ("Financials", "balance_sheets")
        | ("Financials", "cash_flow_statements")
        | ("Financials", "income_statements")
        | ("Financials", "ratios") => schema_for!(financials::Params),
        ("Tickers", "types") => return Ok(no_params()),
        _ => {
            return Err(Error::Custom(format!("Unknown endpoint: {module}::{endpoint}")));
        }
    };

    serde_json::to_value(schema).map_err(|e| Error::Custom(format!("Failed to serialize schema: {e}")))
}

/// Call an endpoint with arguments
async fn call_endpoint<Client: Request>(client: &Polygon<Client>, params: &Value) -> Result<Value> {
    let module = params
        .get("module")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::Custom("Missing 'module' parameter".to_string()))?;

    let endpoint = params
        .get("endpoint")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::Custom("Missing 'endpoint' parameter".to_string()))?;

    let arguments = params
        .get("arguments")
        .ok_or_else(|| Error::Custom("Missing 'arguments' parameter".to_string()))?;

    // Build the full endpoint enum from module + endpoint + arguments
    let endpoint_enum = build_endpoint(module, endpoint, arguments)?;

    // Execute the endpoint (returns raw JSON string)
    let json_str = match endpoint_enum {
        Endpoint::Tickers(t) => call_tickers(client, t).await?,
        Endpoint::Aggs(a) => call_aggs(client, a).await?,
        Endpoint::Financials(f) => call_financials(client, f).await?,
    };

    // Parse to JSON Value
    serde_json::from_str(&json_str).map_err(|e| Error::Custom(format!("Failed to parse JSON: {e}")))
}

/// Build endpoint enum from components
fn build_endpoint(module: &str, endpoint: &str, arguments: &Value) -> Result<Endpoint> {
    match module {
        "Tickers" => {
            let tickers = match endpoint {
                "all" => Tickers::All(serde_json::from_value(arguments.clone())?),
                "details" => Tickers::Details(serde_json::from_value(arguments.clone())?),
                "related" => Tickers::Related(serde_json::from_value(arguments.clone())?),
                "types" => Tickers::Types,
                "events" => Tickers::Events(serde_json::from_value(arguments.clone())?),
                "news" => Tickers::News(serde_json::from_value(arguments.clone())?),
                _ => {
                    return Err(Error::Custom(format!("Unknown Tickers endpoint: {endpoint}")));
                }
            };
            Ok(Endpoint::Tickers(tickers))
        }
        "Aggs" => {
            let aggs = match endpoint {
                "aggregates" => Aggs::Aggregates(serde_json::from_value(arguments.clone())?),
                "previous_close" => Aggs::PreviousClose(serde_json::from_value(arguments.clone())?),
                "grouped_daily" => Aggs::GroupedDaily(serde_json::from_value(arguments.clone())?),
                "daily_open_close" => Aggs::DailyOpenClose(serde_json::from_value(arguments.clone())?),
                _ => {
                    return Err(Error::Custom(format!("Unknown Aggs endpoint: {endpoint}")));
                }
            };
            Ok(Endpoint::Aggs(aggs))
        }
        "Financials" => {
            let financials = match endpoint {
                "balance_sheets" => Financials::BalanceSheets(serde_json::from_value(arguments.clone())?),
                "cash_flow_statements" => Financials::CashFlowStatements(serde_json::from_value(arguments.clone())?),
                "income_statements" => Financials::IncomeStatements(serde_json::from_value(arguments.clone())?),
                "ratios" => Financials::Ratios(serde_json::from_value(arguments.clone())?),
                _ => {
                    return Err(Error::Custom(format!("Unknown Financials endpoint: {endpoint}")));
                }
            };
            Ok(Endpoint::Financials(financials))
        }
        _ => Err(Error::Custom(format!("Unknown module: {module}"))),
    }
}

// ===== Implementation functions  =====

async fn call_tickers<Client: Request>(client: &Polygon<Client>, endpoint: Tickers) -> Result<String> {
    use crate::rest::raw;

    match endpoint {
        Tickers::All(p) => {
            let mut q = raw::tickers::all(client);
            if let Some(t) = p.ticker {
                q = q.ticker(t);
            }
            if let Some(tt) = p.ticker_type {
                q = q.ticker_type(tt);
            }
            if let Some(m) = p.market {
                q = q.market(m);
            }
            if let Some(e) = p.exchange {
                q = q.exchange(e);
            }
            if let Some(l) = p.limit {
                q = q.limit(l);
            }
            if let Some(s) = p.sort {
                q = q.sort(s);
            }
            if let Some(o) = p.order {
                q = q.order(o);
            }
            q.get().await
        }
        Tickers::Details(p) => {
            let mut q = raw::tickers::details(client, &p.ticker);
            if let Some(d) = p.date {
                q = q.date(d);
            }
            q.get().await
        }
        Tickers::Related(p) => raw::tickers::related(client, &p.ticker).get().await,
        Tickers::Types => raw::tickers::types(client).get().await,
        Tickers::Events(p) => {
            let mut q = raw::tickers::events(client, &p.ticker);
            if let Some(t) = p.types {
                q = q.types(t);
            }
            q.get().await
        }
        Tickers::News(p) => {
            let mut q = raw::tickers::news(client);
            if let Some(t) = p.ticker {
                q = q.ticker(t);
            }
            if let Some(l) = p.limit {
                q = q.limit(l);
            }
            if let Some(o) = p.order {
                q = q.order(o);
            }
            q.get().await
        }
    }
}

async fn call_aggs<Client: Request>(client: &Polygon<Client>, endpoint: Aggs) -> Result<String> {
    use crate::rest::raw;

    match endpoint {
        Aggs::Aggregates(p) => {
            let mut q = raw::aggs::aggregates(client, &p.ticker, p.multiplier, p.timespan, &p.from, &p.to);
            if let Some(a) = p.adjusted {
                q = q.adjusted(a);
            }
            if let Some(s) = p.sort {
                q = q.sort(format!("{s:?}").to_lowercase());
            }
            if let Some(l) = p.limit {
                q = q.limit(l);
            }
            q.get().await
        }
        Aggs::PreviousClose(p) => {
            let mut q = raw::aggs::previous_close(client, &p.ticker);
            if let Some(a) = p.adjusted {
                q = q.adjusted(a);
            }
            q.get().await
        }
        Aggs::GroupedDaily(p) => {
            let mut q = raw::aggs::grouped_daily(client, &p.date);
            if let Some(a) = p.adjusted {
                q = q.adjusted(a);
            }
            if let Some(o) = p.include_otc {
                q = q.include_otc(o);
            }
            q.get().await
        }
        Aggs::DailyOpenClose(p) => {
            let mut q = raw::aggs::daily_open_close(client, &p.ticker, &p.date);
            if let Some(a) = p.adjusted {
                q = q.adjusted(a);
            }
            q.get().await
        }
    }
}

async fn call_financials<Client: Request>(client: &Polygon<Client>, endpoint: Financials) -> Result<String> {
    use crate::rest::raw;

    match endpoint {
        Financials::BalanceSheets(p) => {
            let q = raw::financials::balance_sheets(client);
            apply_financial_params(q, p).get().await
        }
        Financials::CashFlowStatements(p) => {
            let q = raw::financials::cash_flow_statements(client);
            apply_financial_params(q, p).get().await
        }
        Financials::IncomeStatements(p) => {
            let q = raw::financials::income_statements(client);
            apply_financial_params(q, p).get().await
        }
        Financials::Ratios(p) => {
            let q = raw::financials::ratios(client);
            apply_financial_params(q, p).get().await
        }
    }
}

fn apply_financial_params<Client: Request>(
    mut q: crate::request::financials::Financials<Client, crate::processor::Raw>,
    p: crate::request::financials::Params,
) -> crate::request::financials::Financials<Client, crate::processor::Raw> {
    if let Some(t) = p.ticker {
        q = q.ticker(t);
    }
    if let Some(c) = p.cik {
        q = q.cik(c);
    }
    if let Some(f) = p.filing_date {
        q = q.filing_date(f);
    }
    if let Some(pr) = p.period_of_report_date {
        q = q.period_of_report_date(pr);
    }
    if let Some(l) = p.limit {
        q = q.limit(l);
    }
    if let Some(o) = p.order {
        q = q.order(o);
    }
    q
}
