use opensearch::{OpenSearch, http::transport::Transport};
use serde_json::json;
use crate::kafka::{LogEvent, MetricEvent, TraceEvent};

pub struct OpenSearchClient {
    client: OpenSearch,
}

impl OpenSearchClient {
    pub fn new() -> Self {
        let transport = Transport::single_node("http://localhost:9200").unwrap();
        let client = OpenSearch::new(transport);
        Self { client }
    }
}

static mut CLIENT: Option<OpenSearchClient> = None;

fn get_client() -> &'static OpenSearchClient {
    unsafe {
        CLIENT.get_or_insert_with(|| OpenSearchClient::new())
    }
}

pub async fn index_log(event: &LogEvent) {
    let client = get_client();
    let index_name = format!("logs-{}-{}", event.agent_id, chrono::Utc::now().format("%Y-%m"));
    
    let doc = json!({
        "timestamp": event.timestamp,
        "level": event.level,
        "message": event.message,
        "service": event.service,
        "agent_id": event.agent_id,
        "source": event.source,
        "trace_id": event.trace_id,
        "@timestamp": chrono::Utc::now().to_rfc3339()
    });

    let _ = client.client
        .index(opensearch::IndexParts::IndexId(&index_name, &uuid::Uuid::new_v4().to_string()))
        .body(doc)
        .send()
        .await;
}

pub async fn index_metric(event: &MetricEvent) {
    let client = get_client();
    let index_name = format!("metrics-{}-{}", event.agent_id, chrono::Utc::now().format("%Y-%m"));
    
    let doc = json!({
        "timestamp": event.timestamp,
        "agent_id": event.agent_id,
        "metric_type": event.metric_type,
        "value": event.value,
        "unit": event.unit,
        "labels": event.labels,
        "@timestamp": chrono::Utc::now().to_rfc3339()
    });

    let _ = client.client
        .index(opensearch::IndexParts::IndexId(&index_name, &uuid::Uuid::new_v4().to_string()))
        .body(doc)
        .send()
        .await;
}

pub async fn index_trace(event: &TraceEvent) {
    let client = get_client();
    let index_name = format!("traces-{}-{}", event.agent_id, chrono::Utc::now().format("%Y-%m"));
    
    let doc = json!({
        "timestamp": event.timestamp,
        "agent_id": event.agent_id,
        "trace_id": event.trace_id,
        "span_id": event.span_id,
        "operation": event.operation,
        "duration_ms": event.duration_ms,
        "status": event.status,
        "@timestamp": chrono::Utc::now().to_rfc3339()
    });

    let _ = client.client
        .index(opensearch::IndexParts::IndexId(&index_name, &uuid::Uuid::new_v4().to_string()))
        .body(doc)
        .send()
        .await;
}

pub async fn search_logs(agent_id: &str, query: Option<&str>, size: usize) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = get_client();
    let index_pattern = format!("logs-{}-*", agent_id);
    
    let search_body = if let Some(q) = query {
        json!({
            "query": {
                "bool": {
                    "must": [
                        {"match": {"message": q}}
                    ]
                }
            },
            "sort": [{"@timestamp": {"order": "desc"}}],
            "size": size
        })
    } else {
        json!({
            "query": {"match_all": {}},
            "sort": [{"@timestamp": {"order": "desc"}}],
            "size": size
        })
    };

    let response = client.client
        .search(opensearch::SearchParts::Index(&[&index_pattern]))
        .body(search_body)
        .send()
        .await?;

    Ok(response.json().await?)
}

pub async fn search_metrics(agent_id: &str, metric_type: Option<&str>, hours: u32) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = get_client();
    let index_pattern = format!("metrics-{}-*", agent_id);
    
    let since = chrono::Utc::now() - chrono::Duration::hours(hours as i64);
    
    let mut must_clauses = vec![
        json!({"range": {"@timestamp": {"gte": since.to_rfc3339()}}})
    ];
    
    if let Some(mt) = metric_type {
        must_clauses.push(json!({"term": {"metric_type": mt}}));
    }

    let search_body = json!({
        "query": {
            "bool": {
                "must": must_clauses
            }
        },
        "sort": [{"@timestamp": {"order": "desc"}}],
        "size": 1000
    });

    let response = client.client
        .search(opensearch::SearchParts::Index(&[&index_pattern]))
        .body(search_body)
        .send()
        .await?;

    Ok(response.json().await?)
}
