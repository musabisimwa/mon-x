use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEvent {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub service: String,
    pub agent_id: String,
    pub trace_id: Option<String>,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetricEvent {
    pub timestamp: String,
    pub agent_id: String,
    pub metric_type: String,
    pub value: f64,
    pub unit: String,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraceEvent {
    pub timestamp: String,
    pub agent_id: String,
    pub trace_id: String,
    pub span_id: String,
    pub operation: String,
    pub duration_ms: u64,
    pub status: String,
}

pub async fn start_consumer() -> Result<(), Box<dyn std::error::Error>> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "ml-monitoring")
        .set("bootstrap.servers", "localhost:9092")
        .set("auto.offset.reset", "latest")
        .create()?;

    // Subscribe to all agent topics using patterns
    consumer.subscribe(&["logs-.*", "metrics-.*", "traces-.*"])?;
    println!("Kafka consumer started for agent topics");

    loop {
        match consumer.recv().await {
            Ok(message) => {
                let topic = message.topic();
                if let Some(payload) = message.payload() {
                    if let Ok(data_str) = std::str::from_utf8(payload) {
                        process_message_by_topic(topic, data_str).await;
                    }
                }
            }
            Err(e) => {
                eprintln!("Kafka error: {}", e);
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

async fn process_message_by_topic(topic: &str, data: &str) {
    match topic {
        t if t.starts_with("logs-") => {
            if let Ok(log_event) = serde_json::from_str::<LogEvent>(data) {
                println!(" Processing log from {}: {}", t, log_event.message);
                crate::opensearch::index_log(&log_event).await;
                crate::ml::analyze_event(&log_event).await;
            }
        },
        t if t.starts_with("metrics-") => {
            if let Ok(metric_event) = serde_json::from_str::<MetricEvent>(data) {
                println!("Processing metric from {}: {} = {}", t, metric_event.metric_type, metric_event.value);
                crate::opensearch::index_metric(&metric_event).await;
                crate::ml::analyze_metric(&metric_event).await;
            }
        },
        t if t.starts_with("traces-") => {
            if let Ok(trace_event) = serde_json::from_str::<TraceEvent>(data) {
                println!(" Processing trace from {}: {}", t, trace_event.operation);
                crate::opensearch::index_trace(&trace_event).await;
            }
        },
        _ => {
            println!("Unknown topic: {}", topic);
        }
    }
}

pub async fn create_agent_topics(agent_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    // For now, topics will be auto-created when first message is sent
    // In production, use AdminClient to pre-create topics
    println!("Topics will be auto-created for agent: {}", agent_id);
    Ok(())
}
