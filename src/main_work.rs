// Rust Project: Real-Time Crypto Trading System with Sentiment Analysis
// Description: This project integrates live crypto market data, sentiment analysis from news headlines, and trade execution.
// Features:
// - Fetch live market data from Binance API
// - Perform sentiment analysis on news headlines using Python
// - Predict price trends
// - Generate alerts and execute trades

use reqwest::blocking::Client;
use serde_json::Value;
use std::process::Command;
use std::collections::HashMap;

// Structure to hold crypto data
#[derive(Debug)]
struct Crypto {
    symbol: String,
    price_change_percent: f64,
    current_price: f64,
}

// Fetch live market data from Binance API
fn fetch_binance_prices() -> Result<Vec<Crypto>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://api.binance.com/api/v3/ticker/24hr";
    let response = client.get(url).send()?.json::<Value>()?;

    let mut cryptos = Vec::new();

    for item in response.as_array().unwrap() {
        let symbol = item["symbol"].as_str().unwrap().to_string();
        let price_change_percent = item["priceChangePercent"].as_str().unwrap().parse::<f64>().unwrap_or(0.0);
        let current_price = item["lastPrice"].as_str().unwrap().parse::<f64>().unwrap_or(0.0);

        if price_change_percent.abs() > 5.0 { // Filter significant movers
            cryptos.push(Crypto {
                symbol,
                price_change_percent,
                current_price,
            });
        }
    }

    Ok(cryptos)
}

// Run sentiment analysis using Python script
fn run_sentiment_analysis(headlines: Vec<&str>) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let output = Command::new("python3")
        .arg("sentiment_analysis.py")
        .arg(headlines.join(","))
        .output()?;

    let sentiments: Vec<f64> = String::from_utf8_lossy(&output.stdout)
        .trim()
        .split(',')
        .map(|s| s.parse::<f64>().unwrap_or(0.0))
        .collect();

    Ok(sentiments)
}

// Decision-making based on price and sentiment
fn decide_trade_action(symbol: &str, price_change_percent: f64, sentiment_score: f64) {
    if price_change_percent > 5.0 && sentiment_score > 0.5 {
        println!("Buy Signal for {}: Price change = {:.2}%, Sentiment = {:.2}", symbol, price_change_percent, sentiment_score);
    } else if price_change_percent < -5.0 && sentiment_score < -0.5 {
        println!("Sell Signal for {}: Price change = {:.2}%, Sentiment = {:.2}", symbol, price_change_percent, sentiment_score);
    } else {
        println!("Hold Signal for {}: Price change = {:.2}%, Sentiment = {:.2}", symbol, price_change_percent, sentiment_score);
    }
}

fn main() {
    // Fetch live crypto data
    match fetch_binance_prices() {
        Ok(cryptos) => {
            println!("Top Movers: {:?}", cryptos);

            // Mock news headlines for sentiment analysis
            let headlines = vec![
                "Bitcoin hits record high!",
                "Ethereum gas fees surge, users concerned.",
                "Market crash: Investors panic over regulations."
            ];

            // Run sentiment analysis
            match run_sentiment_analysis(headlines) {
                Ok(sentiments) => {
                    for (i, crypto) in cryptos.iter().enumerate() {
                        let sentiment_score = sentiments.get(i).unwrap_or(&0.0);
                        decide_trade_action(&crypto.symbol, crypto.price_change_percent, *sentiment_score);
                    }
                }
                Err(e) => println!("Error in sentiment analysis: {}", e),
            }
        }
        Err(e) => println!("Error fetching Binance prices: {}", e),
    }
}

// Python script: sentiment_analysis.py
/*
from textblob import TextBlob
import sys

def analyze_sentiment(headlines):
    sentiments = []
    for headline in headlines.split(','):
        analysis = TextBlob(headline)
        polarity = analysis.sentiment.polarity
        sentiments.append(polarity)
    print(','.join(map(str, sentiments)))

if __name__ == "__main__":
    headlines = sys.argv[1]
    analyze_sentiment(headlines)
*/
