//! Test the tool_use module
use emporium_core::ToolResult;
use polygon::Polygon;
use polygon::tool_use::{ToolCallResult, call_tool, list_tools};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // List all available tools
    println!("Available tools:");
    for tool in list_tools() {
        println!("- {} ({}): {}", tool.id, tool.name, tool.description);
    }
    println!();

    // Test calling a tool (if API key is available)
    if let Ok(api_key) = std::env::var("POLYGON_API_KEY") {
        let client = Polygon::default().with_key(api_key);

        println!("Testing financials endpoint - AAPL income statements:");
        let request = json!({
            "tool": "call_endpoint",
            "params": {
                "module": "Financials",
                "endpoint": "income_statements",
                "arguments": {
                    "ticker": "AAPL",
                    "limit": 5
                }
            }
        });

        match call_tool(&client, request).await {
            Ok(ToolCallResult::DataFrame { data, schema, metadata }) => {
                eprintln!("Data: {}", data);
                eprintln!(
                    "Schema: {}",
                    schema
                        .iter()
                        .map(|c| format!("{} as {} ({})", c.name, c.alias, c.dtype))
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                let emp = emporium_core::ToolResult::columnar(data, schema, metadata.clone());
                match emp {
                    ToolResult::DataFrame(proto) => match proto.to_dataframe() {
                        Ok(df) => println!("Dataframe:\n{}", df.to_string()),
                        Err(e) => println!("Error converting to dataframe: {e}"),
                    },
                    ToolResult::Text(text) => println!("Text: {}", text),
                }
                if let Some(meta) = metadata {
                    println!("\nMetadata: {}", serde_json::to_string_pretty(&meta)?);
                }
            }
            Ok(ToolCallResult::Text(text)) => {
                println!("Success! Response: {}", text);
            }
            Err(e) => println!("Error calling tool: {}", e),
        }
    } else {
        println!("No API key found. Set POLYGON_API_KEY environment variable to test tool calls.");
    }

    Ok(())
}
