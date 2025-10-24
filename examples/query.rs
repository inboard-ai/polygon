//! Example showing the query API for ticker and aggregate endpoints
use polygon::Polygon;
use polygon::query::Execute as _;
use polygon::rest::{aggs, raw, tickers};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Polygon::new()?;

    println!("=== Example 1: Related Tickers (Raw JSON) ===");
    let json = raw::tickers::related(&client, "AAPL").get().await?;
    println!("Related tickers: {json}\n");

    println!("=== Example 2: Ticker Events (Decoded) ===");
    let events = tickers::events(&client, "AAPL").get().await?;
    println!("Events for {}:", events.name);
    println!("  CIK: {}", events.cik);
    println!("  Composite FIGI: {}", events.composite_figi);
    if let Some(event_list) = events.events {
        println!("  Events: {} total", event_list.len());
        for (i, event) in event_list.iter().take(3).enumerate() {
            println!("    {}: {} on {} -> {}", i + 1, event.event_type, event.date, event.ticker_change.ticker);
        }
    }
    println!();

    println!("=== Example 3: Ticker News (Decoded with params) ===");
    let news = tickers::news(&client)
        .param("ticker", "AAPL")
        .param("limit", "5")
        .get()
        .await?;
    println!("Latest news articles: {} total", news.len());
    for (i, article) in news.iter().enumerate() {
        println!("  {}. {}", i + 1, article.title.as_deref().unwrap_or("(no title)"));
        println!("     Author: {}", article.author.as_deref().unwrap_or("Unknown"));
        println!("     Published: {}", article.published_utc.as_deref().unwrap_or("N/A"));
    }
    println!();

    println!("=== Example 4: Previous Close Aggregate (Decoded) ===");
    let close = aggs::previous_close(&client, "AAPL").get().await?;
    for agg in close {
        println!("Previous close for {}:", agg.ticker.as_deref().unwrap_or("N/A"));
        println!("  Open: ${:.2}", agg.open.unwrap_or(0.0));
        println!("  High: ${:.2}", agg.high.unwrap_or(0.0));
        println!("  Low: ${:.2}", agg.low.unwrap_or(0.0));
        println!("  Close: ${:.2}", agg.close.unwrap_or(0.0));
        println!("  Volume: {:.0}", agg.volume.unwrap_or(0.0));
    }
    println!();

    println!("=== Example 5: List Tickers (with params) ===");
    let response = tickers::all(&client)
        .param("limit", "10")
        .param("market", "stocks")
        .get()
        .await?;
    println!("First 10 stock tickers:");
    for (i, t) in response.into_iter().enumerate() {
        println!(
            "  {:>2}. {:>6}  {}",
            i + 1,
            t.ticker.unwrap_or_default(),
            t.name.unwrap_or_default()
        );
    }

    Ok(())
}
