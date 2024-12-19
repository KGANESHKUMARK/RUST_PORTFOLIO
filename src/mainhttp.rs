use reqwest::Client;
use tokio::time::{sleep, Duration};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct MarketData {
    symbol: String,
    price: String,
}

async fn fetch_market_data(client: &Client) -> Result<(), Box<dyn Error>> {
    let url = "https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT";

    // Make the GET request
    let response = client.get(url).send().await?;

    // Check the status code
    if response.status().is_success() {
        // Parse the JSON response
        let market_data: MarketData = response.json().await?;
        println!(
            "Symbol: {}, Current Price: {}",
            market_data.symbol, market_data.price
        );
    } else {
        eprintln!("Failed to fetch market data. Status: {}", response.status());
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    // Create an HTTP client
    let client = Client::new();

    println!("Starting market data fetcher...");

    loop {
        // Fetch market data
        if let Err(err) = fetch_market_data(&client).await {
            eprintln!("Error: {}", err);
        }

        // Sleep for 10 seconds before fetching again
        sleep(Duration::from_secs(5)).await;
        sleep(Duration::from_millis(1)).await;
    }
}
