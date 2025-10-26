//! Example showing table/DataFrame responses from polygon.io

use polygon::Polygon;
use polygon::rest::table;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Polygon::new()?;

    println!("=== Example 1: Tickers as DataFrame ===");
    let df = table::tickers::all(&client).limit("5").market("stocks").get().await?;

    println!("Tickers DataFrame: {df}\n");
    println!("Shape: {:?} rows x {:?} columns\n\n", df.height(), df.width());

    println!("=== Example 2: Previous Close as DataFrame ===");
    let df = table::aggs::previous_close(&client, "AAPL").get().await?;

    println!("Previous close for AAPL: {df}\n\n");

    println!("=== Example 3: Ticker Details as DataFrame ===");
    let df = table::tickers::details(&client, "AAPL").get().await?;

    println!("Ticker details: {df}\n\n");

    println!("=== Example 4: Financial Ratios as DataFrame ===");
    match table::financials::ratios(&client).ticker("AAPL").limit("3").get().await {
        Ok(df) => {
            println!("Financial ratios for AAPL: {df}\n\n");
        }
        Err(e) => {
            println!("Error (may be API entitlement): {e}\n\n");
        }
    }

    println!("=== Example 5: Ticker News as DataFrame ===");
    let df = table::tickers::news(&client).ticker("AAPL").limit("5").get().await?;

    println!("Recent news articles: {df}\n");
    println!("Total articles: {}\n\n", df.height());

    Ok(())
}
