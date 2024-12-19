// Rust Project: Real-Time Crypto Trading System with Deep Learning Prediction
// Description: This project integrates live crypto market data from CoinMarketCap, uses deep learning to predict future trends based on historical data (last 5 years), and executes trades based on the predictions.
// Features:
// - Fetch live market data from CoinMarketCap API
// - Fetch historical data for the last 5 years
// - Deep learning model for future price prediction
// - Generate alerts and execute trades

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
    let symbols_to_check = vec!["XRP"];

    for item in response["data"].as_array().unwrap() {
        let symbol = item["symbol"].as_str().unwrap().to_string();
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

    Ok(cryptos)
}

// Fetch historical data for the last 5 years
fn fetch_historical_data(symbol: &str, api_key: &str) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!(
        "https://pro-api.coinmarketcap.com/v1/cryptocurrency/ohlcv/historical?symbol={}&interval=1d&time_start=2018-01-01",
        symbol
    );

    let response = client
        .get(&url)
        .header("X-CMC_PRO_API_KEY", api_key)
        .send()?
        .json::<Value>()?;

    let mut historical_data = Vec::new();

    if let Some(prices) = response["data"]["quotes"].as_array() {
        for day in prices {
            if let Some(price) = day["quote"]["USD"]["close"].as_f64() {
                historical_data.push(price);
            }
        }
    }

    Ok(historical_data)
}

// Run deep learning prediction using Python script
fn run_deep_learning_prediction(historical_data: Vec<f64>) -> Result<f64, Box<dyn std::error::Error>> {
    let input_data = serde_json::to_string(&historical_data)?;
    let output = Command::new("python3")
        .arg("deep_learning_prediction.py")
        .arg(input_data)
        .output()?;

    if !output.status.success() {
        return Err(format!("Python script failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    let prediction: f64 = String::from_utf8_lossy(&output.stdout).trim().parse()?;
    Ok(prediction)
}

// Decision-making based on prediction
fn decide_trade_action(symbol: &str, current_price: f64, predicted_price: f64) {
    if predicted_price > current_price * 1.05 {
        println!("Buy Signal for {}: Predicted price = {:.2}, Current price = {:.2}", symbol, predicted_price, current_price);
    } else if predicted_price < current_price * 0.95 {
        println!("Sell Signal for {}: Predicted price = {:.2}, Current price = {:.2}", symbol, predicted_price, current_price);
    } else {
        println!("Hold Signal for {}: Predicted price = {:.2}, Current price = {:.2}", symbol, predicted_price, current_price);
    }
}

fn main() {
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
                    Err(e) => println!("Error in prediction for {}: {}", crypto.symbol, e),
                }
            }
        }
        Err(e) => println!("Error fetching CoinMarketCap data: {}", e),
    }
}

// Python script: deep_learning_prediction.py
/*
import sys
import json
import numpy as np
from tensorflow.keras.models import Sequential
from tensorflow.keras.layers import Dense, LSTM

def predict_price(historical_data):
    # Prepare data
    data = np.array(historical_data).reshape((1, len(historical_data), 1))

    # Build a simple LSTM model
    model = Sequential()
    model.add(LSTM(100, activation='relu', input_shape=(len(historical_data), 1)))
    model.add(Dense(1))
    model.compile(optimizer='adam', loss='mse')

    # Mock training (replace with pre-trained model loading for production)
    model.fit(data, np.array([historical_data[-1]]), epochs=20, verbose=0)

    # Predict the next price
    predicted_price = model.predict(data, verbose=0)[0][0]
    return predicted_price

if __name__ == "__main__":
    try:
        historical_data = json.loads(sys.argv[1])
        predicted_price = predict_price(historical_data)
        print(predicted_price)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
*/