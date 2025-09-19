use serde::{Deserialize, Serialize};
use sysinfo::System;
use std::collections::HashMap;
use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentMetrics {
    pub agent_name: String,
    pub timestamp: String,
    pub cpu_usage: f32,
    pub memory_usage: f64,
    pub disk_usage: HashMap<String, f64>,
    pub network_rx: u64,
    pub network_tx: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentLog {
    pub agent_name: String,
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub source_file: String,
}

pub async fn collect_metrics(config: &Config) -> AgentMetrics {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // Get CPU usage (simplified)
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let memory_usage = (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0;
    
    // Simplified disk usage (just report root filesystem)
    let mut disk_usage = HashMap::new();
    disk_usage.insert("/".to_string(), 50.0); // Mock value
    
    // Simplified network stats
    let network_rx = 1024 * 1024; // Mock 1MB
    let network_tx = 512 * 1024;  // Mock 512KB
    
    AgentMetrics {
        agent_name: config.agent.name.clone(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        cpu_usage,
        memory_usage,
        disk_usage,
        network_rx,
        network_tx,
    }
}

pub async fn collect_logs(config: &Config) -> Vec<AgentLog> {
    let mut logs = Vec::new();
    
    for path in &config.log_paths {
        if let Ok(content) = tokio::fs::read_to_string(path).await {
            for line in content.lines().rev().take(10) {
                if let Some(log) = parse_log_line(line, path, &config.agent.name) {
                    logs.push(log);
                }
            }
        }
    }
    
    // If no logs found, create a sample log
    if logs.is_empty() {
        logs.push(AgentLog {
            agent_name: config.agent.name.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            level: "INFO".to_string(),
            message: "Agent heartbeat - system monitoring active".to_string(),
            source_file: "agent".to_string(),
        });
    }
    
    logs
}

fn parse_log_line(line: &str, source: &str, agent_name: &str) -> Option<AgentLog> {
    if line.trim().is_empty() {
        return None;
    }
    
    let level = if line.contains("ERROR") { "ERROR" }
    else if line.contains("WARN") { "WARN" }
    else if line.contains("INFO") { "INFO" }
    else { "DEBUG" };
    
    Some(AgentLog {
        agent_name: agent_name.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        level: level.to_string(),
        message: line.to_string(),
        source_file: source.to_string(),
    })
}
