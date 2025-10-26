//! Example: Progressive API discovery for LLMs
//!
//! Demonstrates how an AI agent can explore and use the Polygon API without
//! prior knowledge of its structure.

use polygon::Polygon;
use polygon::tool_use;
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
    println!("Available tools:\n{}\n", serde_json::to_string_pretty(&tools)?);

    // Step 2: What modules exist?
    let modules = tool_use::call_tool(
        &client,
        json!({
            "tool": "list_modules",
            "params": {}
        }),
    )
    .await?;
    println!("API modules:\n{}\n", serde_json::to_string_pretty(&modules)?);

    // Step 3: What endpoints are in the Aggs module?
    let endpoints = tool_use::call_tool(
        &client,
        json!({
            "tool": "list_endpoints",
            "params": { "module": "Aggs" }
        }),
    )
    .await?;
    println!("Aggs endpoints:\n{}\n", serde_json::to_string_pretty(&endpoints)?);

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
    println!("Aggregates schema:\n{}\n", serde_json::to_string_pretty(&schema)?);

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
    println!("AAPL aggregates:\n{}\n", serde_json::to_string_pretty(&data)?);

    Ok(())
}
