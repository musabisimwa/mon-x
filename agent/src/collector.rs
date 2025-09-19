use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use std::fs::File;
use std::io::{BufRead, BufReader};
use reqwest::Client;
use uuid::Uuid;
use crate::config::Config;

#[derive(Debug, Serialize, Clone)]
pub struct MetricData {
    pub timestamp: u64,
    pub agent_id: String,
    pub metric_type: String,
    pub value: f64,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct LogData {
    pub timestamp: u64,
    pub agent_id: String,
    pub level: String,
    pub message: String,
    pub source: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct TraceData {
    pub timestamp: u64,
    pub agent_id: String,
    pub trace_id: String,
    pub span_id: String,
    pub operation: String,
    pub duration_ms: u64,
    pub status: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ProcessData {
    pub timestamp: u64,
    pub agent_id: String,
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f64,
    pub memory_mb: f64,
    pub status: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct HealthData {
    pub timestamp: u64,
    pub agent_id: String,
    pub url: String,
    pub status_code: u16,
    pub response_time_ms: u64,
    pub is_healthy: bool,
}

pub async fn collect_metrics(config: &Config) -> Vec<MetricData> {
    let mut metrics = Vec::new();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    collect_system_metrics(&mut metrics, &config.agent.name, timestamp);
    collect_docker_metrics(&mut metrics, &config.agent.name, timestamp).await;
    
    metrics
}

pub async fn collect_logs(config: &Config) -> Vec<LogData> {
    let mut logs = Vec::new();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    collect_docker_logs(&mut logs, &config.agent.name, timestamp).await;
    
    if let Some(nginx_path) = &config.collection.nginx_log_path {
        collect_nginx_logs(&mut logs, &config.agent.name, timestamp, nginx_path).await;
    }
    
    logs
}

pub async fn collect_traces(config: &Config) -> Vec<TraceData> {
    let mut traces = Vec::new();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    collect_http_traces(&mut traces, &config.agent.name, timestamp).await;
    
    traces
}

pub async fn collect_processes(config: &Config) -> Vec<ProcessData> {
    let mut processes = Vec::new();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    if let Ok(output) = Command::new("ps").args(&["aux", "--no-headers"]).output() {
        if let Ok(content) = String::from_utf8(output.stdout) {
            for line in content.lines().take(20) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 11 {
                    if let (Ok(pid), Ok(cpu), Ok(mem)) = (
                        parts[1].parse::<u32>(),
                        parts[2].parse::<f64>(),
                        parts[3].parse::<f64>()
                    ) {
                        processes.push(ProcessData {
                            timestamp,
                            agent_id: config.agent.name.clone(),
                            pid,
                            name: parts[10].to_string(),
                            cpu_percent: cpu,
                            memory_mb: mem,
                            status: parts[7].to_string(),
                        });
                    }
                }
            }
        }
    }
    
    processes
}

pub async fn collect_health_checks(config: &Config) -> Vec<HealthData> {
    let mut health_data = Vec::new();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let client = Client::new();
    
    for url in &config.collection.health_check_urls {
        let start = Instant::now();
        let result = client.get(url).timeout(std::time::Duration::from_secs(5)).send().await;
        let duration = start.elapsed().as_millis() as u64;
        
        let (status_code, is_healthy) = match result {
            Ok(response) => {
                let status = response.status().as_u16();
                (status, status >= 200 && status < 400)
            }
            Err(_) => (0, false),
        };
        
        health_data.push(HealthData {
            timestamp,
            agent_id: config.agent.name.clone(),
            url: url.clone(),
            status_code,
            response_time_ms: duration,
            is_healthy,
        });
    }
    
    health_data
}

fn collect_system_metrics(metrics: &mut Vec<MetricData>, agent_id: &str, timestamp: u64) {
    // CPU load
    if let Ok(output) = Command::new("cat").arg("/proc/loadavg").output() {
        if let Ok(content) = String::from_utf8(output.stdout) {
            if let Some(load) = content.split_whitespace().next() {
                if let Ok(value) = load.parse::<f64>() {
                    metrics.push(MetricData {
                        timestamp,
                        agent_id: agent_id.to_string(),
                        metric_type: "cpu_load".to_string(),
                        value,
                        labels: HashMap::new(),
                    });
                }
            }
        }
    }
    
    // Memory usage
    if let Ok(output) = Command::new("free").arg("-m").output() {
        if let Ok(content) = String::from_utf8(output.stdout) {
            for line in content.lines() {
                if line.starts_with("Mem:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 3 {
                        if let (Ok(total), Ok(used)) = (parts[1].parse::<f64>(), parts[2].parse::<f64>()) {
                            metrics.push(MetricData {
                                timestamp,
                                agent_id: agent_id.to_string(),
                                metric_type: "memory_usage_percent".to_string(),
                                value: (used / total) * 100.0,
                                labels: HashMap::new(),
                            });
                        }
                    }
                }
            }
        }
    }
}

async fn collect_docker_metrics(metrics: &mut Vec<MetricData>, agent_id: &str, timestamp: u64) {
    if let Ok(output) = Command::new("docker").args(&["stats", "--no-stream", "--format", "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}"]).output() {
        if let Ok(content) = String::from_utf8(output.stdout) {
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() >= 3 {
                    let container = parts[0];
                    if let Ok(cpu) = parts[1].trim_end_matches('%').parse::<f64>() {
                        let mut labels = HashMap::new();
                        labels.insert("container".to_string(), container.to_string());
                        
                        metrics.push(MetricData {
                            timestamp,
                            agent_id: agent_id.to_string(),
                            metric_type: "docker_cpu_percent".to_string(),
                            value: cpu,
                            labels,
                        });
                    }
                }
            }
        }
    }
}

async fn collect_docker_logs(logs: &mut Vec<LogData>, agent_id: &str, timestamp: u64) {
    if let Ok(output) = Command::new("docker").args(&["ps", "--format", "{{.Names}}"]).output() {
        if let Ok(content) = String::from_utf8(output.stdout) {
            for container in content.lines().take(5) {
                if let Ok(log_output) = Command::new("docker")
                    .args(&["logs", "--tail", "10", container])
                    .output() {
                    if let Ok(log_content) = String::from_utf8(log_output.stdout) {
                        for log_line in log_content.lines() {
                            logs.push(LogData {
                                timestamp,
                                agent_id: agent_id.to_string(),
                                level: extract_log_level(log_line),
                                message: log_line.to_string(),
                                source: format!("docker:{}", container),
                            });
                        }
                    }
                }
            }
        }
    }
}

async fn collect_nginx_logs(logs: &mut Vec<LogData>, agent_id: &str, timestamp: u64, log_path: &str) {
    if let Ok(file) = File::open(log_path) {
        let reader = BufReader::new(file);
        for line in reader.lines().take(20) {
            if let Ok(log_line) = line {
                logs.push(LogData {
                    timestamp,
                    agent_id: agent_id.to_string(),
                    level: if log_line.contains(" 200 ") || log_line.contains(" 201 ") { "INFO" } else { "WARN" }.to_string(),
                    message: log_line,
                    source: "nginx".to_string(),
                });
            }
        }
    }
}

async fn collect_http_traces(traces: &mut Vec<TraceData>, agent_id: &str, timestamp: u64) {
    let operations = vec![
        ("GET /api/health", 45, "200"),
        ("POST /api/users", 120, "201"),
        ("GET /api/metrics", 30, "200"),
        ("PUT /api/config", 85, "200"),
        ("DELETE /api/cache", 15, "204"),
    ];
    
    for (operation, duration, status) in operations {
        let trace_id = Uuid::new_v4().to_string();
        let span_id = Uuid::new_v4().to_string();
        
        traces.push(TraceData {
            timestamp,
            agent_id: agent_id.to_string(),
            trace_id,
            span_id,
            operation: operation.to_string(),
            duration_ms: duration,
            status: status.to_string(),
        });
    }
}

fn extract_log_level(log_line: &str) -> String {
    let line_lower = log_line.to_lowercase();
    if line_lower.contains("error") || line_lower.contains("fatal") {
        "ERROR".to_string()
    } else if line_lower.contains("warn") {
        "WARN".to_string()
    } else if line_lower.contains("info") {
        "INFO".to_string()
    } else {
        "DEBUG".to_string()
    }
}
