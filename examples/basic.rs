//! Basic usage example for the polygon.io client
use polygon::Polygon;
use polygon::query::Execute;
use polygon::rest::aggs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Polygon::new()?
    // .with_key("your_api_key_here")
    ;

    // Get previous day's close for AAPL
    let result = aggs::previous_close(&client, "AAPL").get().await?;
    println!("{:?}", result);

    Ok(())
}
