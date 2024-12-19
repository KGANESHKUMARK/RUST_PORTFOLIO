# import sys
# import json
# import numpy as np
# from tensorflow.keras.models import Sequential
# from tensorflow.keras.layers import Dense, LSTM

# def predict_price(historical_data):
#     # Prepare data
#     data = np.array(historical_data).reshape((1, len(historical_data), 1))

#     # Build a simple LSTM model
#     model = Sequential()
#     model.add(LSTM(50, activation='relu', input_shape=(len(historical_data), 1)))
#     model.add(Dense(1))
#     model.compile(optimizer='adam', loss='mse')

#     # Mock training (in a real scenario, you'd load a pre-trained model)
#     model.fit(data, np.array([historical_data[-1]]), epochs=10, verbose=0)

#     # Predict the next price
#     predicted_price = model.predict(data, verbose=0)[0][0]
#     return predicted_price

# if __name__ == "__main__":
#     historical_data = json.loads(sys.argv[1])
#     predicted_price = predict_price(historical_data)
#     print(predicted_price)


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
    historical_data = json.loads(sys.argv[1])
    predicted_price = predict_price(historical_data)
    print(predicted_price)