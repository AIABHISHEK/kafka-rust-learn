use rskafka::{
    client::{
        ClientBuilder, 
        partition::UnknownTopicHandling, // <--- Added import
    },
};

#[tokio::main]
async fn main() {
    let connection = "localhost:9092";
    
    // 1. Connect
    let client = ClientBuilder::new(vec![connection.to_string()])
        .build()
        .await
        .expect("Failed to connect");

    let topic_name = "docker-topic";

    // 2. Setup Partition Client
    let partition_client = client
        .partition_client(
            topic_name, 
            0, 
            UnknownTopicHandling::Error
        )
        .await
        .expect("Failed to find partition");

    println!("🎧 Consumer started! Waiting for messages...");

    // 3. Manual Fetch Loop
    // We start at offset 0 (beginning)
    let mut current_offset = 0; 

    loop {
        // Args: (start_offset, min_bytes..max_bytes, max_wait_ms)
        let response = partition_client
            .fetch_records(current_offset, 1..4096, 500) 
            .await;

        match response {
            Ok((records, _high_watermark)) => {
                for record_data in records {
                    // Update offset
                    current_offset = record_data.offset + 1;

                    let record = record_data.record;
                    let value = match record.value {
                        Some(v) => String::from_utf8_lossy(&v).to_string(),
                        None => "no value".to_string(),
                    };
                    
                    let key = match record.key {
                        Some(k) => String::from_utf8_lossy(&k).to_string(),
                        None => "no key".to_string(),
                    };

                    println!("📥 Received Key: '{}' | Value: '{}' | Offset: {}", key, value, record_data.offset);
                }
            }
            Err(e) => {
                eprintln!("Error fetching records: {:?}", e);
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
    }
}
