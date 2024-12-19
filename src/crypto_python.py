import numpy as np
import pandas as pd
import tensorflow as tf
from binance.client import Client
from sklearn.preprocessing import MinMaxScaler
import smtplib
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart

# Binance API setup
api_key = 'your_api_key'
api_secret = 'your_api_secret'
client = Client(api_key, api_secret)

# Function to get historical data from Binance
def get_historical_data(symbol, interval, start_str, end_str=None):
    klines = client.get_historical_klines(symbol, interval, start_str, end_str)
    df = pd.DataFrame(klines, columns=['timestamp', 'open', 'high', 'low', 'close', 'volume', 'close_time', 'quote_asset_volume', 'number_of_trades', 'taker_buy_base_asset_volume', 'taker_buy_quote_asset_volume', 'ignore'])
    df['timestamp'] = pd.to_datetime(df['timestamp'], unit='ms')
    df.set_index('timestamp', inplace=True)
    df = df[['open', 'high', 'low', 'close', 'volume']]
    return df

# Preprocessing function for the data
def preprocess_data(df):
    df = df.astype('float32')
    df['close'] = df['close'] / df['close'].max()  # Normalize 'close' prices
    return df

# Function to create dataset for LSTM training
def create_dataset(df, window_size=60):
    X, y = [], []
    for i in range(window_size, len(df)):
        X.append(df.iloc[i-window_size:i, 3].values)  # 'close' prices
        y.append(df.iloc[i, 3])  # Next 'close'
    X = np.array(X)
    y = np.array(y)
    return X, y

# Build the LSTM model
def build_model(X_train):
    model = tf.keras.Sequential([
        tf.keras.layers.LSTM(units=50, return_sequences=True, input_shape=(X_train.shape[1], 1)),
        tf.keras.layers.LSTM(units=50),
        tf.keras.layers.Dense(units=1)
    ])
    model.compile(optimizer='adam', loss='mean_squared_error')
    return model

# Send alert via email
def send_alert(subject, body):
    sender_email = "your_email@example.com"
    receiver_email = "receiver_email@example.com"
    password = "your_email_password"

    message = MIMEMultipart()
    message['From'] = sender_email
    message['To'] = receiver_email
    message['Subject'] = subject
    message.attach(MIMEText(body, 'plain'))

    server = smtplib.SMTP('smtp.example.com', 587)
    server.starttls()
    server.login(sender_email, password)
    text = message.as_string()
    server.sendmail(sender_email, receiver_email, text)
    server.quit()

# Function to get live data
def get_live_data(symbol):
    klines = client.get_klines(symbol=symbol, interval=Client.KLINE_INTERVAL_1MINUTE, limit=60)
    df_live = pd.DataFrame(klines, columns=['timestamp', 'open', 'high', 'low', 'close', 'volume', 'close_time', 'quote_asset_volume', 'number_of_trades', 'taker_buy_base_asset_volume', 'taker_buy_quote_asset_volume', 'ignore'])
    df_live['timestamp'] = pd.to_datetime(df_live['timestamp'], unit='ms')
    df_live.set_index('timestamp', inplace=True)
    df_live = df_live[['close']].astype('float32')
    return df_live

# Main function to train, predict, and trade
def main():
    # Get historical data for training
    symbol = 'BTCUSDT'
    interval = '1h'
    start_str = '1 year ago UTC'
    historical_data = get_historical_data(symbol, interval, start_str)
    historical_data = preprocess_data(historical_data)
    
    # Prepare training data
    X, y = create_dataset(historical_data)
    X = X.reshape(X.shape[0], X.shape[1], 1)  # Reshape for LSTM input
    train_size = int(len(X) * 0.8)
    X_train, X_test = X[:train_size], X[train_size:]
    y_train, y_test = y[:train_size], y[train_size:]
    
    # Build and train the LSTM model
    model = build_model(X_train)
    model.fit(X_train, y_train, epochs=10, batch_size=32)

    # Get live data and predict the next price
    live_data = get_live_data(symbol)
    live_data_scaled = live_data['close'] / historical_data['close'].max()  # Normalize

    # Predict next price movement
    prediction_input = live_data_scaled.values.reshape(1, -1, 1)
    predicted_price = model.predict(prediction_input)

    current_price = live_data['close'].iloc[-1]
    print(f"Current price: {current_price}, Predicted next price: {predicted_price}")

    # If the predicted price is higher than the current price, issue a buy signal
    if predicted_price > current_price:
        print("Buy signal detected! Buying...")
        client.order_market_buy(symbol=symbol, quantity=0.01)
        
        # Send an alert email
        subject = "Buy Signal Detected"
        body = f"Buy signal detected for {symbol} at {current_price}. Predicted next price: {predicted_price}"
        send_alert(subject, body)
    else:
        print("No buy signal.")

# Run the main function
if __name__ == "__main__":
    main()
