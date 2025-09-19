use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Mutex, LazyLock};

#[derive(Deserialize)]
pub struct LogQuery {
    q: Option<String>,
    from: Option<usize>,
    size: Option<usize>,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    data: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Agent {
    pub name: String,
    pub last_seen: String,
    pub capabilities: HashMap<String, bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricData {
    pub timestamp: u64,
    pub agent_id: String,
    pub metric_type: String,
    pub value: f64,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogData {
    pub timestamp: u64,
    pub agent_id: String,
    pub level: String,
    pub message: String,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraceData {
    pub timestamp: u64,
    pub agent_id: String,
    pub trace_id: String,
    pub span_id: String,
    pub operation: String,
    pub duration_ms: u64,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessData {
    pub timestamp: u64,
    pub agent_id: String,
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f64,
    pub memory_mb: f64,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HealthData {
    pub timestamp: u64,
    pub agent_id: String,
    pub url: String,
    pub status_code: u16,
    pub response_time_ms: u64,
    pub is_healthy: bool,
}

static AGENTS: LazyLock<Mutex<HashMap<String, Agent>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
static METRICS: LazyLock<Mutex<Vec<MetricData>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static LOGS: LazyLock<Mutex<Vec<LogData>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static TRACES: LazyLock<Mutex<Vec<TraceData>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static PROCESSES: LazyLock<Mutex<Vec<ProcessData>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static HEALTH_DATA: LazyLock<Mutex<Vec<HealthData>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub async fn get_logs(query: web::Query<LogQuery>) -> Result<HttpResponse> {
    let search_query = query.q.as_deref().unwrap_or("*");
    let from = query.from.unwrap_or(0);
    let size = query.size.unwrap_or(50);
    
    match crate::opensearch::search_logs(search_query, from, size).await {
        Ok(results) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: results,
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "error": e.to_string()
        }))),
    }
}

pub async fn get_metrics() -> Result<HttpResponse> {
    // Mock metrics - in production, calculate from OpenSearch aggregations
    let metrics = json!({
        "total_logs": 125000,
        "error_rate": 0.023,
        "avg_response_time": 145.7,
        "active_services": 12,
        "anomalies_detected": crate::ml::get_anomalies().len()
    });
    
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: metrics,
    }))
}

pub async fn get_anomalies() -> Result<HttpResponse> {
    let anomalies = crate::ml::get_anomalies();
    
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: anomalies,
    }))
}

pub async fn register_agent(agent_data: web::Json<serde_json::Value>) -> Result<HttpResponse> {
    let name = agent_data["name"].as_str().unwrap_or("unknown").to_string();
    let capabilities = agent_data["capabilities"].as_object()
        .map(|obj| obj.iter().map(|(k, v)| (k.clone(), v.as_bool().unwrap_or(false))).collect())
        .unwrap_or_default();
    
    let agent = Agent {
        name: name.clone(),
        last_seen: chrono::Utc::now().to_rfc3339(),
        capabilities,
    };
    
    AGENTS.lock().unwrap().insert(name, agent);
    
    Ok(HttpResponse::Ok().json(json!({"success": true, "message": "Agent registered"})))
}

pub async fn receive_agent_metrics(metrics: web::Json<Vec<MetricData>>) -> Result<HttpResponse> {
    let metrics_data = metrics.into_inner();
    let metrics_len = metrics_data.len();
    
    let mut metrics_store = METRICS.lock().unwrap();
    metrics_store.extend(metrics_data);
    
    // Keep only last 1000 metrics
    let store_len = metrics_store.len();
    if store_len > 1000 {
        metrics_store.drain(0..store_len - 1000);
    }
    
    println!("Received {} metrics", metrics_len);
    
    Ok(HttpResponse::Ok().json(json!({"success": true})))
}

pub async fn receive_agent_logs(logs: web::Json<Vec<LogData>>) -> Result<HttpResponse> {
    let logs_data = logs.into_inner();
    let logs_len = logs_data.len();
    
    let mut logs_store = LOGS.lock().unwrap();
    
    // Convert and store logs
    for log in &logs_data {
        let log_event = crate::kafka::LogEvent {
            timestamp: chrono::DateTime::from_timestamp(log.timestamp as i64, 0)
                .unwrap_or_default()
                .to_rfc3339(),
            level: log.level.clone(),
            message: log.message.clone(),
            service: log.agent_id.clone(),
            trace_id: None,
        };
        
        // Process through ML pipeline
        crate::ml::analyze_event(&log_event).await;
        
        // Store in OpenSearch
        crate::opensearch::index_log(&log_event).await;
    }
    
    logs_store.extend(logs_data);
    
    // Keep only last 1000 logs
    let store_len = logs_store.len();
    if store_len > 1000 {
        logs_store.drain(0..store_len - 1000);
    }
    
    println!("Received {} logs", logs_len);
    
    Ok(HttpResponse::Ok().json(json!({"success": true})))
}

pub async fn receive_agent_traces(traces: web::Json<Vec<TraceData>>) -> Result<HttpResponse> {
    let traces_data = traces.into_inner();
    let traces_len = traces_data.len();
    
    let mut traces_store = TRACES.lock().unwrap();
    traces_store.extend(traces_data);
    
    // Keep only last 1000 traces
    let store_len = traces_store.len();
    if store_len > 1000 {
        traces_store.drain(0..store_len - 1000);
    }
    
    println!("Received {} traces", traces_len);
    
    Ok(HttpResponse::Ok().json(json!({"success": true})))
}

pub async fn receive_agent_processes(processes: web::Json<Vec<ProcessData>>) -> Result<HttpResponse> {
    let processes_data = processes.into_inner();
    let processes_len = processes_data.len();
    
    let mut processes_store = PROCESSES.lock().unwrap();
    processes_store.extend(processes_data);
    
    // Keep only last 500 process snapshots
    let store_len = processes_store.len();
    if store_len > 500 {
        processes_store.drain(0..store_len - 500);
    }
    
    println!("Received {} process entries", processes_len);
    
    Ok(HttpResponse::Ok().json(json!({"success": true})))
}

pub async fn receive_agent_health(health_data: web::Json<Vec<HealthData>>) -> Result<HttpResponse> {
    let health_data_inner = health_data.into_inner();
    let health_len = health_data_inner.len();
    
    let mut health_store = HEALTH_DATA.lock().unwrap();
    health_store.extend(health_data_inner);
    
    // Keep only last 200 health checks
    let store_len = health_store.len();
    if store_len > 200 {
        health_store.drain(0..store_len - 200);
    }
    
    println!("Received {} health checks", health_len);
    
    Ok(HttpResponse::Ok().json(json!({"success": true})))
}

pub async fn get_processes() -> Result<HttpResponse> {
    let processes = PROCESSES.lock().unwrap().clone();
    
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: processes,
    }))
}

pub async fn get_health_status() -> Result<HttpResponse> {
    let health_data = HEALTH_DATA.lock().unwrap().clone();
    
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: health_data,
    }))
}

pub async fn get_agents() -> Result<HttpResponse> {
    let agents: Vec<Agent> = AGENTS.lock().unwrap().values().cloned().collect();
    
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: agents,
    }))
}
