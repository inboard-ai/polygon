//! Example showing the fluent query API for ticker endpoints

use polygon::Polygon;
use polygon::rest::raw::tickers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Polygon::new()?;

    // Simple - no optional params
    let json = tickers::related(&client, "AAPL").get().await?;
    println!("Related tickers: {}\n", json);

    // Single param
    let json = tickers::all(&client).param("limit", 10).get().await?;
    println!("First 10 tickers: {}\n", json);

    // Multiple params - fluent builder pattern
    let json = tickers::all(&client)
        .param("ticker", "AAPL")
        .param("active", true)
        .param("limit", 5)
        .get()
        .await?;
    println!("Filtered tickers: {}\n", json);

    // // Complex query with many params
    // let json = tickers::all_tickers(&client)
    //     .param("market", "stocks")
    //     .param("type", "CS")
    //     .param("limit", 100)
    //     .param("sort", "ticker")
    //     .get()
    //     .await?;
    // println!("Common stocks: {}\n", json);

    // // Ticker details with optional date param
    // let json = tickers::ticker_details(&client, "AAPL")
    //     .param("date", "2023-01-01")
    //     .get()
    //     .await?;
    // println!("AAPL details on 2023-01-01: {}\n", json);

    Ok(())
}
