use serde::Serialize;
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

#[derive(Debug, Serialize, Clone)]
pub struct HttpCallData {
    pub timestamp: u64,
    pub agent_id: String,
    pub method: String,
    pub url: String,
    pub status_code: u16,
    pub duration_ms: u64,
    pub request_size: u64,
    pub response_size: u64,
}

#[derive(Debug, Serialize, Clone)]
pub struct DatabaseQueryData {
    pub timestamp: u64,
    pub agent_id: String,
    pub db_type: String,
    pub query: String,
    pub duration_ms: u64,
    pub rows_affected: u64,
    pub error: Option<String>,
}

pub async fn collect_metrics(config: &Config) -> Vec<MetricData> {
    let mut metrics = Vec::new();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    collect_system_metrics(&mut metrics, &config.agent.name, timestamp);
    collect_docker_metrics(&mut metrics, &config.agent.name, timestamp).await;
    collect_custom_app_metrics(&mut metrics, &config.agent.name, timestamp);
    
    metrics
}

fn collect_custom_app_metrics(metrics: &mut Vec<MetricData>, agent_id: &str, timestamp: u64) {
    // Custom application metrics - can be extended per application
    if let Ok(output) = Command::new("ps").args(&["aux"]).output() {
        if let Ok(content) = String::from_utf8(output.stdout) {
            let app_processes = content.lines()
                .filter(|line| line.contains("node") || line.contains("python") || line.contains("java"))
                .count();
            
            metrics.push(MetricData {
                timestamp, agent_id: agent_id.to_string(),
                metric_type: "app_processes_count".to_string(), 
                value: app_processes as f64,
                labels: HashMap::new(),
            });
        }
    }
    
    // Check for specific application ports
    if let Ok(output) = Command::new("ss").args(&["-tuln"]).output() {
        if let Ok(content) = String::from_utf8(output.stdout) {
            let listening_ports = content.lines()
                .filter(|line| line.contains("LISTEN"))
                .count();
            
            metrics.push(MetricData {
                timestamp, agent_id: agent_id.to_string(),
                metric_type: "listening_ports_count".to_string(),
                value: listening_ports as f64,
                labels: HashMap::new(),
            });
        }
    }
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

pub async fn collect_http_calls(config: &Config) -> Vec<HttpCallData> {
    let mut http_calls = Vec::new();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    // Monitor network connections for HTTP traffic
    if let Ok(output) = Command::new("ss").args(&["-tuln"]).output() {
        if let Ok(content) = String::from_utf8(output.stdout) {
            for line in content.lines() {
                if line.contains(":80") || line.contains(":443") || line.contains(":8080") {
                    // Simplified HTTP call tracking - in production use eBPF or tcpdump
                    http_calls.push(HttpCallData {
                        timestamp,
                        agent_id: config.agent.name.clone(),
                        method: "GET".to_string(),
                        url: "detected_connection".to_string(),
                        status_code: 200,
                        duration_ms: 0,
                        request_size: 0,
                        response_size: 0,
                    });
                }
            }
        }
    }
    
    http_calls
}

pub async fn collect_database_queries(config: &Config) -> Vec<DatabaseQueryData> {
    let mut queries = Vec::new();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    // Monitor database connections
    if let Ok(output) = Command::new("ss").args(&["-tuln"]).output() {
        if let Ok(content) = String::from_utf8(output.stdout) {
            for line in content.lines() {
                if line.contains(":5432") || line.contains(":3306") || line.contains(":27017") {
                    let db_type = if line.contains(":5432") { "postgresql" }
                                 else if line.contains(":3306") { "mysql" }
                                 else { "mongodb" };
                    
                    queries.push(DatabaseQueryData {
                        timestamp,
                        agent_id: config.agent.name.clone(),
                        db_type: db_type.to_string(),
                        query: "connection_detected".to_string(),
                        duration_ms: 0,
                        rows_affected: 0,
                        error: None,
                    });
                }
            }
        }
    }
    
    queries
}

fn collect_system_metrics(metrics: &mut Vec<MetricData>, agent_id: &str, timestamp: u64) {
    // CPU per core
    if let Ok(content) = std::fs::read_to_string("/proc/stat") {
        for (i, line) in content.lines().enumerate() {
            if line.starts_with("cpu") && line != "cpu" {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 8 {
                    let total: u64 = parts[1..8].iter().filter_map(|s| s.parse::<u64>().ok()).sum();
                    let idle: u64 = parts[4].parse().unwrap_or(0);
                    let usage = if total > 0 { ((total - idle) as f64 / total as f64) * 100.0 } else { 0.0 };
                    
                    metrics.push(MetricData {
                        timestamp, agent_id: agent_id.to_string(),
                        metric_type: "cpu_core_usage".to_string(), value: usage,
                        labels: [("core".to_string(), (i-1).to_string())].into(),
                    });
                }
            }
        }
    }

    // Memory details
    if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
        let mut mem_data = HashMap::new();
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(value) = parts[1].parse::<f64>() {
                    mem_data.insert(parts[0].trim_end_matches(':'), value * 1024.0);
                }
            }
        }
        
        if let (Some(&total), Some(&free), Some(&buffers), Some(&cached)) = 
            (mem_data.get("MemTotal"), mem_data.get("MemFree"), 
             mem_data.get("Buffers"), mem_data.get("Cached")) {
            let used = total - free - buffers - cached;
            metrics.push(MetricData {
                timestamp, agent_id: agent_id.to_string(),
                metric_type: "memory_used".to_string(), value: used,
                labels: HashMap::new(),
            });
        }
        
        if let (Some(&swap_total), Some(&swap_free)) = 
            (mem_data.get("SwapTotal"), mem_data.get("SwapFree")) {
            metrics.push(MetricData {
                timestamp, agent_id: agent_id.to_string(),
                metric_type: "swap_used".to_string(), value: swap_total - swap_free,
                labels: HashMap::new(),
            });
        }
    }

    // Load averages
    if let Ok(content) = std::fs::read_to_string("/proc/loadavg") {
        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() >= 3 {
            for (i, &load) in parts[0..3].iter().enumerate() {
                if let Ok(value) = load.parse::<f64>() {
                    metrics.push(MetricData {
                        timestamp, agent_id: agent_id.to_string(),
                        metric_type: "load_average".to_string(), value,
                        labels: [("period".to_string(), ["1m","5m","15m"][i].to_string())].into(),
                    });
                }
            }
        }
    }

    // Disk I/O
    if let Ok(content) = std::fs::read_to_string("/proc/diskstats") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 14 && !parts[2].starts_with("loop") {
                if let (Ok(reads), Ok(writes)) = (parts[5].parse::<f64>(), parts[9].parse::<f64>()) {
                    metrics.push(MetricData {
                        timestamp, agent_id: agent_id.to_string(),
                        metric_type: "disk_reads".to_string(), value: reads,
                        labels: [("device".to_string(), parts[2].to_string())].into(),
                    });
                    metrics.push(MetricData {
                        timestamp, agent_id: agent_id.to_string(),
                        metric_type: "disk_writes".to_string(), value: writes,
                        labels: [("device".to_string(), parts[2].to_string())].into(),
                    });
                }
            }
        }
    }

    // Network interfaces
    if let Ok(content) = std::fs::read_to_string("/proc/net/dev") {
        for line in content.lines().skip(2) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 17 {
                let iface = parts[0].trim_end_matches(':');
                if let (Ok(rx_bytes), Ok(tx_bytes)) = (parts[1].parse::<f64>(), parts[9].parse::<f64>()) {
                    metrics.push(MetricData {
                        timestamp, agent_id: agent_id.to_string(),
                        metric_type: "network_rx_bytes".to_string(), value: rx_bytes,
                        labels: [("interface".to_string(), iface.to_string())].into(),
                    });
                    metrics.push(MetricData {
                        timestamp, agent_id: agent_id.to_string(),
                        metric_type: "network_tx_bytes".to_string(), value: tx_bytes,
                        labels: [("interface".to_string(), iface.to_string())].into(),
                    });
                }
            }
        }
    }

    // File system usage
    if let Ok(output) = Command::new("df").args(&["-B1"]).output() {
        if let Ok(content) = String::from_utf8(output.stdout) {
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 && parts[5].starts_with('/') {
                    if let (Ok(total), Ok(used)) = (parts[1].parse::<f64>(), parts[2].parse::<f64>()) {
                        metrics.push(MetricData {
                            timestamp, agent_id: agent_id.to_string(),
                            metric_type: "filesystem_used_bytes".to_string(), value: used,
                            labels: [("mountpoint".to_string(), parts[5].to_string())].into(),
                        });
                        metrics.push(MetricData {
                            timestamp, agent_id: agent_id.to_string(),
                            metric_type: "filesystem_usage_percent".to_string(), 
                            value: if total > 0.0 { (used / total) * 100.0 } else { 0.0 },
                            labels: [("mountpoint".to_string(), parts[5].to_string())].into(),
                        });
                    }
                }
            }
        }
    }

    // Temperature sensors
    if let Ok(entries) = std::fs::read_dir("/sys/class/thermal") {
        for entry in entries.flatten() {
            if entry.file_name().to_string_lossy().starts_with("thermal_zone") {
                let temp_path = entry.path().join("temp");
                if let Ok(temp_str) = std::fs::read_to_string(&temp_path) {
                    if let Ok(temp) = temp_str.trim().parse::<f64>() {
                        metrics.push(MetricData {
                            timestamp, agent_id: agent_id.to_string(),
                            metric_type: "temperature_celsius".to_string(), value: temp / 1000.0,
                            labels: [("sensor".to_string(), entry.file_name().to_string_lossy().to_string())].into(),
                        });
                    }
                }
            }
        }
    }

    // GPU metrics (NVIDIA)
    if let Ok(output) = Command::new("nvidia-smi")
        .args(&["--query-gpu=utilization.gpu,memory.used,memory.total,temperature.gpu", "--format=csv,noheader,nounits"])
        .output() {
        if let Ok(content) = String::from_utf8(output.stdout) {
            for (i, line) in content.lines().enumerate() {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 4 {
                    if let (Ok(util), Ok(mem_used), Ok(mem_total), Ok(temp)) = 
                        (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>(), 
                         parts[2].trim().parse::<f64>(), parts[3].trim().parse::<f64>()) {
                        let gpu_id = i.to_string();
                        metrics.push(MetricData {
                            timestamp, agent_id: agent_id.to_string(),
                            metric_type: "gpu_utilization".to_string(), value: util,
                            labels: [("gpu".to_string(), gpu_id.clone())].into(),
                        });
                        metrics.push(MetricData {
                            timestamp, agent_id: agent_id.to_string(),
                            metric_type: "gpu_memory_used".to_string(), value: mem_used * 1024.0 * 1024.0,
                            labels: [("gpu".to_string(), gpu_id.clone())].into(),
                        });
                        metrics.push(MetricData {
                            timestamp, agent_id: agent_id.to_string(),
                            metric_type: "gpu_temperature".to_string(), value: temp,
                            labels: [("gpu".to_string(), gpu_id)].into(),
                        });
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
