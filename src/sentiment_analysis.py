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