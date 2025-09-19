use reqwest::Client;
use serde_json::json;
use crate::config::Config;
use crate::collector::{AgentMetrics, AgentLog};

pub async fn register_agent(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("{}/api/agents/register", config.agent.server_url);
    
    let payload = json!({
        "name": config.agent.name,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "capabilities": {
            "metrics": config.collection.metrics,
            "logs": config.collection.logs,
            "resources": config.collection.resources
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

pub async fn send_data(
    config: &Config,
    metrics: AgentMetrics,
    logs: Vec<AgentLog>
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // Send metrics
    if config.collection.metrics {
        let url = format!("{}/api/agents/metrics", config.agent.server_url);
        let _ = client.post(&url).json(&metrics).send().await;
    }
    
    // Send logs
    if config.collection.logs && !logs.is_empty() {
        let url = format!("{}/api/agents/logs", config.agent.server_url);
        let _ = client.post(&url).json(&logs).send().await;
    }
    
    Ok(())
}
