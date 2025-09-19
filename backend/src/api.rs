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

static AGENTS: LazyLock<Mutex<HashMap<String, Agent>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
static AGENT_METRICS: LazyLock<Mutex<Vec<AgentMetrics>>> = LazyLock::new(|| Mutex::new(Vec::new()));

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

pub async fn receive_agent_metrics(metrics: web::Json<AgentMetrics>) -> Result<HttpResponse> {
    let mut agent_metrics = AGENT_METRICS.lock().unwrap();
    agent_metrics.push(metrics.into_inner());
    
    // Keep only last 1000 metrics
    let len = agent_metrics.len();
    if len > 1000 {
        agent_metrics.drain(0..len - 1000);
    }
    
    Ok(HttpResponse::Ok().json(json!({"success": true})))
}

pub async fn receive_agent_logs(logs: web::Json<Vec<AgentLog>>) -> Result<HttpResponse> {
    // Convert agent logs to standard log format and process
    for log in logs.iter() {
        let log_event = crate::kafka::LogEvent {
            timestamp: log.timestamp.clone(),
            level: log.level.clone(),
            message: log.message.clone(),
            service: log.agent_name.clone(),
            trace_id: None,
        };
        
        // Process through ML pipeline
        crate::ml::analyze_event(&log_event).await;
        
        // Store in OpenSearch
        crate::opensearch::index_log(&log_event).await;
    }
    
    Ok(HttpResponse::Ok().json(json!({"success": true})))
}

pub async fn get_agents() -> Result<HttpResponse> {
    let agents: Vec<Agent> = AGENTS.lock().unwrap().values().cloned().collect();
    
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: agents,
    }))
}
