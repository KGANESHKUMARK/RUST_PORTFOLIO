use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::OwnedMessage;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;
use serde::{Deserialize, Serialize};
use tokio::stream::StreamExt;
use log::{info, warn, error};
use env_logger;

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    id: u32,
    item_id: u32,
    quantity: i32,
    timestamp: String,
}

async fn produce_transaction(producer: &FutureProducer, transaction: &Transaction) {
    let payload = serde_json::to_string(transaction).unwrap();
    let record = FutureRecord::to("portfolio_transactions")
        .payload(&payload)
        .key(&transaction.id.to_string());

    match producer.send(record, 0).await {
        Ok(delivery) => info!("Sent: {:?}", delivery),
        Err((e, _)) => error!("Error: {:?}", e),
    }
}

async fn consume_transactions(consumer: &StreamConsumer) {
    let mut message_stream = consumer.start();

    while let Some(message) = message_stream.next().await {
        match message {
            Ok(m) => {
                let payload = m.payload_view::<str>().unwrap().unwrap();
                let transaction: Transaction = serde_json::from_str(payload).unwrap();
                info!("Received: {:?}", transaction);
            }
            Err(e) => error!("Error: {:?}", e),
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation error");

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "portfolio_group")
        .set("bootstrap.servers", "localhost:9092")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()
        .expect("Consumer creation error");

    consumer
        .subscribe(&["portfolio_transactions"])
        .expect("Can't subscribe to specified topic");

    let transaction = Transaction {
        id: 1,
        item_id: 101,
        quantity: 10,
        timestamp: "2023-01-01T12:00:00Z".to_string(),
    };

    produce_transaction(&producer, &transaction).await;
    consume_transactions(&consumer).await;
}