use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEvent {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub service: String,
    pub trace_id: Option<String>,
}

pub async fn start_consumer() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "ml-monitoring")
        .set("bootstrap.servers", "localhost:9092")
        .set("auto.offset.reset", "latest")
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&["logs"])
        .expect("Can't subscribe to topic");

    loop {
        match consumer.recv().await {
            Ok(message) => {
                if let Some(payload) = message.payload() {
                    if let Ok(log_str) = std::str::from_utf8(payload) {
                        if let Ok(log_event) = serde_json::from_str::<LogEvent>(log_str) {
                            process_log_event(log_event).await;
                        }
                    }
                }
            }
            Err(e) => eprintln!("Kafka error: {}", e),
        }
    }
}

async fn process_log_event(event: LogEvent) {
    // Store in OpenSearch
    crate::opensearch::index_log(&event).await;
    
    // Send to ML pipeline
    crate::ml::analyze_event(&event).await;
}
