use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

// Binance API credentials
const API_KEY: &str = "fPT2rPyfYusMmcfspbvq3jPIOF5sgu1Mp2acfS5YQ15fKYZHeQ8angup8sVaJTrL";
const API_SECRET: &str = "RwutX21ytmCtAREdh592iFL38ksxrnW1qVjwlqsXzgwS0qO4QJ9bLn7JnLUOmG8j";

// Struct for Binance's response
#[derive(Serialize, Deserialize, Debug)]
struct Kline {
    open_time: u64,
    close: String,
    high: String,
    low: String,
    open: String,
    volume: String,
}

// Function to create a signature for Binance API requests
fn sign_request(params: HashMap<String, String>, secret: &str) -> String {
    let mut query_string = params.iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<String>>()
        .join("&");

    query_string.push_str(secret);

    let mut hmac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    hmac.update(query_string.as_bytes());
    let result = hmac.finalize();
    let signature = hex::encode(result.into_bytes());

    signature
}

// Function to fetch historical data from Binance
async fn get_historical_data(symbol: &str, interval: &str, limit: u32) -> Result<Vec<Kline>, reqwest::Error> {
    let url = format!(
        "https://api.binance.com/api/v1/klines?symbol={}&interval={}&limit={}",
        symbol, interval, limit
    );

    let response = Client::new().get(&url).send().await?;
    let data: Vec<Vec<serde_json::Value>> = response.json().await?;

    let klines = data.into_iter().map(|kline| Kline {
        open_time: kline[0].as_u64().unwrap(),
        close: kline[4].as_str().unwrap().to_string(),
        high: kline[2].as_str().unwrap().to_string(),
        low: kline[3].as_str().unwrap().to_string(),
        open: kline[1].as_str().unwrap().to_string(),
        volume: kline[5].as_str().unwrap().to_string(),
    }).collect();

    Ok(klines)
}

// Function to place a market buy order
async fn place_order(symbol: &str, quantity: f64) -> Result<String, reqwest::Error> {
    let params = HashMap::from([
        ("symbol".to_string(), symbol.to_string()),
        ("side".to_string(), "BUY".to_string()),
        ("type".to_string(), "MARKET".to_string()),
        ("quantity".to_string(), quantity.to_string()),
        ("timestamp".to_string(), (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string())),
    ]);

    let signature = sign_request(params.clone(), API_SECRET);
    let mut params_with_signature = params.clone();
    params_with_signature.insert("signature".to_string(), signature);

    let url = "https://api.binance.com/api/v3/order";
    let response = Client::new()
        .post(url)
        .header("X-MBX-APIKEY", API_KEY)
        .json(&params_with_signature)
        .send()
        .await?;

    Ok(response.text().await?)
}

#[tokio::main]
async fn main() {
    let symbol = "BTCUSDT";
    let interval = "1h";
    let limit = 100;

    match get_historical_data(symbol, interval, limit).await {
        Ok(klines) => {
            for kline in klines {
                println!("Kline: {:?}", kline);
            }
        }
        Err(err) => eprintln!("Error fetching historical data: {}", err),
    }

    // Example of placing an order
    let quantity = 0.001; // Example quantity to buy
    match place_order(symbol, quantity).await {
        Ok(response) => println!("Order response: {}", response),
        Err(err) => eprintln!("Error placing order: {}", err),
    }
}

use tch::{Tensor, Device, nn, nn::Module};

fn predict_with_model(model_path: &str, input_data: Vec<f32>) -> Tensor {
    // Load the pre-trained model
    let vs = nn::VarStore::new(Device::cuda_if_available());
    let model = nn::seq()
        .add(nn::linear(vs.root(), input_data.len() as i64, 1, Default::default()));

    let input_tensor = Tensor::of_slice(&input_data).view((1, input_data.len() as i64));

    // Make the prediction
    let prediction = model.forward(&input_tensor);
    prediction
}