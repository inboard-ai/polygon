//! Example showing typed responses from polygon.io

use polygon::Polygon;
use polygon::rest::tickers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Polygon::new()?;

    // Raw data
    let json = tickers::related(&client, "AAPL").get().await?;
    println!("[JSON] Related tickers: {json}\n");

    // Decoded into typed structs
    let data = tickers::all(&client)
        .limit(10)
        .exchange("XNYS")
        .decoded() // decode it into types
        .get()
        .await?;

    println!("[Decoded] First ticker: {:?} {:?}", data[0].ticker, data[0].name);

    Ok(())
}
