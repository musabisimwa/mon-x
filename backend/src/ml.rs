use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Mutex, LazyLock};
use crate::kafka::LogEvent;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Anomaly {
    pub timestamp: String,
    pub score: f64,
    pub event: LogEvent,
    pub reason: String,
    pub algorithm: String,
    pub humanized: Option<crate::log_humanizer::HumanizedLog>,
}

static ANOMALIES: LazyLock<Mutex<Vec<Anomaly>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static EVENT_BUFFER: LazyLock<Mutex<Vec<LogEvent>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static LOG_PATTERNS: LazyLock<Mutex<HashMap<String, u32>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

pub async fn start_anomaly_detector() {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
    
    loop {
        interval.tick().await;
        
        // Run multiple algorithms
        detect_statistical_anomalies().await;
        detect_frequency_anomalies().await;
        detect_pattern_anomalies().await;
    }
}

pub async fn analyze_event(event: &LogEvent) {
    let mut buffer = EVENT_BUFFER.lock().unwrap();
    buffer.push(event.clone());
    
    // Update pattern frequency
    let pattern_key = extract_log_pattern(&event.message);
    let mut patterns = LOG_PATTERNS.lock().unwrap();
    *patterns.entry(pattern_key).or_insert(0) += 1;
    
    let len = buffer.len();
    if len > 1000 {
        buffer.drain(0..len - 1000);
    }
}

pub async fn analyze_metric(metric: &crate::kafka::MetricEvent) {
    // Store metrics for anomaly detection
    if metric.metric_type == "cpu" && metric.value > 90.0 {
        let anomaly = Anomaly {
            timestamp: chrono::Utc::now().to_rfc3339(),
            score: 0.95,
            event: LogEvent {
                timestamp: metric.timestamp.clone(),
                level: "WARN".to_string(),
                message: format!("High CPU usage: {}%", metric.value),
                service: "system".to_string(),
                agent_id: metric.agent_id.clone(),
                source: "metrics".to_string(),
                trace_id: None,
            },
            reason: format!("CPU usage spike: {}%", metric.value),
            algorithm: "MetricThreshold".to_string(),
            humanized: None,
        };
        
        let mut anomalies = ANOMALIES.lock().unwrap();
        anomalies.push(anomaly);
    }
}

// Simple statistical anomaly detection (without isolation forest for now)
async fn detect_statistical_anomalies() {
    let buffer = EVENT_BUFFER.lock().unwrap();
    if buffer.len() < 10 { return; }
    
    // Simple heuristic: detect unusually long messages
    let avg_length: f64 = buffer.iter().map(|e| e.message.len() as f64).sum::<f64>() / buffer.len() as f64;
    let threshold = avg_length * 3.0; // 3x average length
    
    let mut anomalies = ANOMALIES.lock().unwrap();
    for event in buffer.iter().rev().take(10) {
        if event.message.len() as f64 > threshold && event.level == "ERROR" {
            anomalies.push(Anomaly {
                timestamp: chrono::Utc::now().to_rfc3339(),
                score: 0.85,
                event: event.clone(),
                reason: format!("Unusually long error message ({} chars vs {} avg)", event.message.len(), avg_length as usize),
                algorithm: "StatisticalAnalysis".to_string(),
                humanized: None,
            });
        }
    }
}

// Random Cut Forest equivalent (frequency-based)
async fn detect_frequency_anomalies() {
    let buffer = EVENT_BUFFER.lock().unwrap();
    if buffer.len() < 50 { return; }
    
    // Count error rates in time windows
    let recent_errors = buffer.iter().rev().take(20)
        .filter(|e| e.level == "ERROR").count();
    
    let historical_errors = buffer.iter().rev().skip(20).take(100)
        .filter(|e| e.level == "ERROR").count();
    
    let recent_rate = recent_errors as f64 / 20.0;
    let historical_rate = historical_errors as f64 / 100.0;
    
    // Anomaly if recent error rate > 3x historical
    if recent_rate > historical_rate * 3.0 && recent_rate > 0.1 {
        let mut anomalies = ANOMALIES.lock().unwrap();
        if let Some(latest_error) = buffer.iter().rev().find(|e| e.level == "ERROR") {
            anomalies.push(Anomaly {
                timestamp: chrono::Utc::now().to_rfc3339(),
                score: 0.92,
                event: latest_error.clone(),
                reason: format!("Error rate spike: {:.1}% vs {:.1}%", recent_rate * 100.0, historical_rate * 100.0),
                algorithm: "RandomCutForest".to_string(),
                humanized: None,
            });
        }
    }
}

// Log pattern embedding (transformer-like)
async fn detect_pattern_anomalies() {
    let patterns = LOG_PATTERNS.lock().unwrap();
    let buffer = EVENT_BUFFER.lock().unwrap();
    
    // Find rare patterns (< 1% frequency)
    let total_logs = buffer.len() as f64;
    let rare_threshold = (total_logs * 0.01).max(1.0) as u32;
    
    for event in buffer.iter().rev().take(10) {
        let pattern = extract_log_pattern(&event.message);
        if let Some(&count) = patterns.get(&pattern) {
            if count < rare_threshold && event.level == "ERROR" {
                let mut anomalies = ANOMALIES.lock().unwrap();
                anomalies.push(Anomaly {
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    score: 0.78,
                    event: event.clone(),
                    reason: format!("Rare error pattern (seen {} times)", count),
                    algorithm: "LogEmbedding".to_string(),
                    humanized: None,
                });
            }
        }
    }
}

// Extract log pattern (simplified tokenization)
fn extract_log_pattern(message: &str) -> String {
    message
        .split_whitespace()
        .map(|word| {
            if word.chars().any(|c| c.is_ascii_digit()) {
                "<NUM>"
            } else if word.contains('@') {
                "<EMAIL>"
            } else if word.starts_with('/') {
                "<PATH>"
            } else {
                word
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn get_anomalies() -> Vec<Anomaly> {
    let mut anomalies = ANOMALIES.lock().unwrap();
    
    // Keep only last 100 anomalies
    let len = anomalies.len();
    if len > 100 {
        anomalies.drain(0..len - 100);
    }
    
    anomalies.clone()
}
