//! Example showing typed responses from polygon.io

use polygon::Polygon;
use polygon::query::Execute as _;
use polygon::rest::aggs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Polygon::new()?;

    // Get typed data directly - no manual decoding needed!
    let close = aggs::previous_close(&client, "AAPL").get().await?;

    println!("Previous close data for AAPL:");
    for agg in close {
        println!("  Ticker: {:?}", agg.ticker);
        println!("  Open: {:?}", agg.open);
        println!("  High: {:?}", agg.high);
        println!("  Low: {:?}", agg.low);
        println!("  Close: {:?}", agg.close);
        println!("  Volume: {:?}", agg.volume);
        println!("  VWAP: {:?}", agg.vwap);
        println!("  Timestamp: {:?}", agg.timestamp);
    }

    Ok(())
}
