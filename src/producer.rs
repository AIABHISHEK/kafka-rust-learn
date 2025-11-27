use rskafka::{
    client::{
        ClientBuilder, 
        partition::{Compression, UnknownTopicHandling}, 
    },
    record::Record,
};
use std::time::Duration;
use chrono::Utc;

#[tokio::main]
async fn main() {
    let connection = "localhost:9092";
    
    // 1. Connect
    let client = ClientBuilder::new(vec![connection.to_string()])
        .build()
        .await
        .expect("Failed to connect to Kafka");

    let topic_name = "docker-topic"; 
    
    // 2. Get a client for Partition 0
    let partition_client = client
        .partition_client(
            topic_name, 
            0, 
            UnknownTopicHandling::Error
        )
        .await
        .expect("Failed to find partition");

    println!("🚀 Producer started! Sending messages to '{}'", topic_name);

    for i in 0..10 {
        let value = format!("Action event number {}", i).into_bytes();
        let key = format!("user-{}", i).into_bytes();

        // 3. Create Record
        let record = Record {
            key: Some(key),
            value: Some(value),
            headers: Default::default(),
            timestamp: Utc::now(),
        };

        // 4. Send
        partition_client
            .produce(vec![record], Compression::NoCompression)
            .await
            .expect("Failed to send");

        println!("✅ Sent message {}", i);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
