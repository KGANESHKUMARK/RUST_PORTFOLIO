use reqwest::blocking::Client;
use serde_json::Value;
use std::process::{Command, Stdio};
use std::io::Write;

// Structure to hold crypto data
#[derive(Debug)]
struct Crypto {
    symbol: String,
    current_price: f64,
    historical_data: Vec<f64>,
}

// Fetch live market data from CoinMarketCap API
fn fetch_coinmarketcap_data(api_key: &str) -> Result<Vec<Crypto>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest";
    let response = client
        .get(url)
        .header("X-CMC_PRO_API_KEY", api_key)
        .send()?
        .json::<Value>()?;

    let mut cryptos = Vec::new();
    let symbols_to_check = vec!["BTC", "ETH", "XRP"];

    if let Some(data) = response["data"].as_array() {
        for item in data {
            let symbol = item["symbol"].as_str().unwrap_or("").to_string();
            if symbols_to_check.contains(&symbol.as_str()) {
                let current_price = item["quote"]["USD"]["price"].as_f64().unwrap_or(0.0);

                // Fetch historical data
                let historical_data = fetch_historical_data(&symbol, api_key)?;

                cryptos.push(Crypto {
                    symbol,
                    current_price,
                    historical_data,
                });
            }
        }
    } else {
        println!("No data found in response");
    }

    Ok(cryptos)
}

// Fetch historical data for a given cryptocurrency symbol
fn fetch_historical_data(symbol: &str, api_key: &str) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/historical?symbol={}&convert=USD", symbol); // Updated API endpoint
    let response = client
        .get(&url)
        .header("X-CMC_PRO_API_KEY", api_key)
        .send()?
        .json::<Value>()?;

    let mut historical_data = Vec::new();

    if let Some(data) = response["data"]["quotes"].as_array() {
        for item in data {
            let price = item["quote"]["USD"]["close"].as_f64().unwrap_or(0.0);
            historical_data.push(price);
        }
    } else {
        println!("No historical data found for {}", symbol);
    }

    Ok(historical_data)
}

// Run deep learning prediction (mocked for demonstration)
fn run_deep_learning_prediction(historical_data: Vec<f64>) -> Result<f64, Box<dyn std::error::Error>> {
    // Mock prediction logic
    let predicted_price = historical_data.iter().sum::<f64>() / historical_data.len() as f64;
    Ok(predicted_price)
}

// Decide trade action based on prediction
fn decide_trade_action(symbol: &str, current_price: f64, predicted_price: f64) {
    if predicted_price > current_price * 1.05 {
        println!("Buy Signal for {}: Predicted price = {:.2}, Current price = {:.2}", symbol, predicted_price, current_price);
    } else if predicted_price < current_price * 0.95 {
        println!("Sell Signal for {}: Predicted price = {:.2}, Current price = {:.2}", symbol, predicted_price, current_price);
    } else {
        println!("Hold Signal for {}: Predicted price = {:.2}, Current price = {:.2}", symbol, predicted_price, current_price);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "6f87fdfd-e0f4-41c2-ab60-b404707fb828"; // Replace with your API key

    // Fetch live crypto data
    match fetch_coinmarketcap_data(api_key) {
        Ok(cryptos) => {
            println!("Fetched cryptos: {:?}", cryptos);

            for crypto in cryptos {
                // Run deep learning prediction for each crypto
                match run_deep_learning_prediction(crypto.historical_data) {
                    Ok(predicted_price) => {
                        decide_trade_action(&crypto.symbol, crypto.current_price, predicted_price);
                    }
                    Err(e) => {
                        println!("Error in prediction for {}: {}", crypto.symbol, e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Error fetching data: {}", e);
        }
    }

    Ok(())
}