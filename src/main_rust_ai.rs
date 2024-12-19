// Rust Project: Portfolio and Transaction Trading System
// Description: This project simulates a basic portfolio and transaction trading system using Rust.
// Features:
// - Manage assets in a portfolio
// - Execute buy and sell transactions
// - Track profits and losses
// - Simple moving average (SMA) trading strategy

use std::collections::HashMap;

// Structure representing a single asset in the portfolio
#[derive(Debug)]
struct Asset {
    symbol: String,
    quantity: f64,
    average_cost: f64, // Weighted average cost
}

impl Asset {
    fn new(symbol: &str) -> Self {
        Asset {
            symbol: symbol.to_string(),
            quantity: 0.0,
            average_cost: 0.0,
        }
    }
}

// Structure for the Portfolio
#[derive(Debug)]
struct Portfolio {
    cash_balance: f64,
    assets: HashMap<String, Asset>,
}

impl Portfolio {
    // Initialize a new portfolio
    fn new(initial_cash: f64) -> Self {
        Portfolio {
            cash_balance: initial_cash,
            assets: HashMap::new(),
        }
    }

    // Buy an asset
    fn buy(&mut self, symbol: &str, price: f64, quantity: f64) {
        let cost = price * quantity;
        if self.cash_balance >= cost {
            self.cash_balance -= cost;

            let asset = self.assets.entry(symbol.to_string()).or_insert(Asset::new(symbol));
            asset.average_cost = (asset.average_cost * asset.quantity + cost) / (asset.quantity + quantity);
            asset.quantity += quantity;

            println!("Bought {} of {} at ${} each. Remaining cash: ${}", quantity, symbol, price, self.cash_balance);
        } else {
            println!("Insufficient cash to buy {} of {}.", quantity, symbol);
        }
    }

    // Sell an asset
    fn sell(&mut self, symbol: &str, price: f64, quantity: f64) {
        if let Some(asset) = self.assets.get_mut(symbol) {
            if asset.quantity >= quantity {
                let proceeds = price * quantity;
                self.cash_balance += proceeds;
                asset.quantity -= quantity;
                if asset.quantity == 0.0 {
                    self.assets.remove(symbol);
                }

                println!("Sold {} of {} at ${} each. New cash balance: ${}", quantity, symbol, price, self.cash_balance);
            } else {
                println!("Not enough quantity to sell {} of {}.", quantity, symbol);
            }
        } else {
            println!("Asset {} not found in portfolio.", symbol);
        }
    }

    // Display the portfolio
    fn display(&self) {
        println!("\nPortfolio Summary:");
        println!("Cash Balance: ${}", self.cash_balance);
        for asset in self.assets.values() {
            println!("Asset: {}, Quantity: {}, Avg Cost: ${}", asset.symbol, asset.quantity, asset.average_cost);
        }
    }
}

// SMA Trading Strategy
fn simple_moving_average(data: &[f64], window_size: usize) -> Vec<f64> {
    let mut sma = Vec::new();
    for i in 0..=(data.len() - window_size) {
        let avg: f64 = data[i..i + window_size].iter().sum::<f64>() / window_size as f64;
        sma.push(avg);
    }
    sma
}

fn main() {
    // Initialize portfolio
    let mut portfolio = Portfolio::new(10000.0);

    // Mock price data for a stock
    let prices = vec![100.0, 102.0, 101.0, 103.0, 104.0, 105.0, 106.0];
    let sma_window = 3;

    // Calculate SMA
    let sma = simple_moving_average(&prices, sma_window);
    println!("Price Data: {:?}", prices);
    println!("{}-Day SMA: {:?}\n", sma_window, sma);

    // Execute trading based on SMA crossover
    for (i, &price) in prices.iter().enumerate().skip(sma_window - 1) {
        if i > sma_window - 1 {
            let prev_price = prices[i - 1];
            let prev_sma = sma[i - sma_window];

            if prev_price < prev_sma && price > sma[i - sma_window + 1] {
                portfolio.buy("AAPL", price, 10.0);
            } else if prev_price > prev_sma && price < sma[i - sma_window + 1] {
                portfolio.sell("AAPL", price, 10.0);
            }
        }
    }

    // Display final portfolio
    portfolio.display();
}
