//! Example demonstrating the type-safe query API
use polygon::request::aggs;
use polygon::request::common::Timespan;

#[tokio::main]
async fn main() -> Result<(), polygon::Error> {
    let client = polygon::Polygon::new()?;

    let json = aggs::aggregates(&client, "AAPL", 1, Timespan::Day, "2024-01-01", "2024-01-05")
        .adjusted(true)
        .limit(5)
        .get()
        .await?;

    println!("1. Raw JSON (first 200 chars): {}...\n", &json[..json.len().min(200)]);

    let df = aggs::Aggregates::new(&client, "AAPL", 1, Timespan::Day, "2024-01-01", "2024-01-05")
        .adjusted(true)
        .limit("5")
        .as_dataframe() // change processor type and get a DataFrame
        .get()
        .await?;

    println!("2. DataFrame:\n{df}\n");

    Ok(())
}
