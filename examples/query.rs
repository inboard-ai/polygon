//! Example showing the fluent query API for ticker endpoints
use polygon::Polygon;
use polygon::query::Execute as _;
use polygon::rest::raw;
use polygon::rest::tickers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Polygon::new()?;

    // Simple - no optional params
    let json = raw::tickers::related(&client, "AAPL").get().await?;
    println!("Related tickers (JSON): {}\n", json);

    // Single param
    let response = tickers::all(&client).param("limit", 10).get().await?;
    println!(
        "First 10 tickers (decoded):\n{}\n",
        response
            .into_iter()
            .enumerate()
            .map(|(i, t)| format!(
                "{i:>2}:  {:>6}  {}",
                t.ticker.unwrap_or_default(),
                t.name.unwrap_or_default()
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );

    // Multiple params
    let response = tickers::all(&client)
        .params([
            ("exchange", "XNYS".to_string()),
            ("limit", 10.to_string()),
            // ("sort", "desc".to_string()),
        ])
        .get()
        .await?;
    println!(
        "NYSE tickers (decoded):\n{}\n",
        response
            .into_iter()
            .enumerate()
            .map(|(i, t)| format!(
                "{i:>2}:  {:>6}  {}",
                t.ticker.unwrap_or_default(),
                t.name.unwrap_or_default()
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );

    Ok(())
}
