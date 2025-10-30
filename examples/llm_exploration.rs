//! Example: Progressive API discovery for LLMs
//!
//! Demonstrates how an AI agent can explore and use the Polygon API without
//! prior knowledge of its structure.

use polygon::Polygon;
use polygon::tool_use::{self, ToolCallResult};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = Polygon::new()?;

    // Step 1: What can I do?
    let tools = tool_use::call_tool(
        &client,
        json!({
            "tool": "list_tools",
            "params": {}
        }),
    )
    .await?;
    if let ToolCallResult::Text(text) = tools {
        println!("Available tools:\n{}\n", text);
    }

    // Step 2: What modules exist?
    let modules = tool_use::call_tool(
        &client,
        json!({
            "tool": "list_modules",
            "params": {}
        }),
    )
    .await?;
    if let ToolCallResult::Text(text) = modules {
        println!("API modules:\n{}\n", text);
    }

    // Step 3: What endpoints are in the Aggs module?
    let endpoints = tool_use::call_tool(
        &client,
        json!({
            "tool": "list_endpoints",
            "params": { "module": "Aggs" }
        }),
    )
    .await?;
    if let ToolCallResult::Text(text) = endpoints {
        println!("Aggs endpoints:\n{}\n", text);
    }

    // Step 4: What parameters does 'aggregates' need?
    let schema = tool_use::call_tool(
        &client,
        json!({
            "tool": "get_endpoint_schema",
            "params": {
                "module": "Aggs",
                "endpoint": "aggregates"
            }
        }),
    )
    .await?;
    if let ToolCallResult::Text(text) = schema {
        println!("Aggregates schema:\n{}\n", text);
    }

    // Step 5: Call the endpoint with parameters
    let data = tool_use::call_tool(
        &client,
        json!({
            "tool": "call_endpoint",
            "params": {
                "module": "Aggs",
                "endpoint": "aggregates",
                "arguments": {
                    "ticker": "AAPL",
                    "multiplier": 1,
                    "timespan": "week",
                    "from": "2024-01-01",
                    "to": "2024-01-31"
                }
            }
        }),
    )
    .await?;
    match data {
        ToolCallResult::DataFrame { data, schema } => {
            println!("AAPL aggregates:\n{}\n", serde_json::to_string_pretty(&data)?);
            println!("Schema: {} columns", schema.len());
        }
        ToolCallResult::Text(text) => {
            println!("AAPL aggregates:\n{}\n", text);
        }
    }

    Ok(())
}
