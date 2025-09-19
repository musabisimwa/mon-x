use reqwest::Client;
use serde_json::json;
use crate::config::Config;
use crate::collector::{MetricData, LogData, TraceData, ProcessData, HealthData};

pub async fn register_agent(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("{}/api/agents/register", config.agent.server_url);
    
    let payload = json!({
        "name": config.agent.name,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "capabilities": {
            "metrics": config.collection.metrics,
            "logs": config.collection.logs,
            "traces": config.collection.traces,
            "processes": config.collection.processes,
            "health": config.collection.health,
            "docker": config.collection.docker
        }
    });
    
    let response = client.post(&url)
        .json(&payload)
        .send()
        .await?;
    
    if response.status().is_success() {
        println!("Agent registered successfully");
    } else {
        println!("Failed to register agent: {}", response.status());
    }
    
    Ok(())
}

pub async fn send_comprehensive_data(
    config: &Config,
    metrics: Vec<MetricData>,
    logs: Vec<LogData>,
    traces: Vec<TraceData>,
    processes: Vec<ProcessData>,
    health_data: Vec<HealthData>,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // Send metrics
    if config.collection.metrics && !metrics.is_empty() {
        let url = format!("{}/api/agents/metrics", config.agent.server_url);
        let _ = client.post(&url).json(&metrics).send().await;
    }
    
    // Send logs (Docker, nginx, system)
    if config.collection.logs && !logs.is_empty() {
        let url = format!("{}/api/agents/logs", config.agent.server_url);
        let _ = client.post(&url).json(&logs).send().await;
    }
    
    // Send traces (HTTP API calls)
    if config.collection.traces && !traces.is_empty() {
        let url = format!("{}/api/agents/traces", config.agent.server_url);
        let _ = client.post(&url).json(&traces).send().await;
    }
    
    // Send process data
    if config.collection.processes && !processes.is_empty() {
        let url = format!("{}/api/agents/processes", config.agent.server_url);
        let _ = client.post(&url).json(&processes).send().await;
    }
    
    // Send health check results
    if config.collection.health && !health_data.is_empty() {
        let url = format!("{}/api/agents/health", config.agent.server_url);
        let _ = client.post(&url).json(&health_data).send().await;
    }
    
    Ok(())
}
