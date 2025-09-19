use opensearch::{OpenSearch, http::transport::Transport};
use serde_json::json;
use crate::kafka::LogEvent;

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
    let index_name = format!("logs-{}", chrono::Utc::now().format("%Y-%m"));
    
    let doc = json!({
        "timestamp": event.timestamp,
        "level": event.level,
        "message": event.message,
        "service": event.service,
        "trace_id": event.trace_id,
        "@timestamp": chrono::Utc::now().to_rfc3339()
    });

    let _ = client.client
        .index(opensearch::IndexParts::IndexId(&index_name, &uuid::Uuid::new_v4().to_string()))
        .body(doc)
        .send()
        .await;
}

pub async fn search_logs(query: &str, from: usize, size: usize) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = get_client();
    let index_pattern = "logs-*";
    
    let search_body = json!({
        "query": {
            "query_string": {
                "query": query
            }
        },
        "from": from,
        "size": size,
        "sort": [{"@timestamp": {"order": "desc"}}]
    });

    let response = client.client
        .search(opensearch::SearchParts::Index(&[index_pattern]))
        .body(search_body)
        .send()
        .await?;

    Ok(response.json().await?)
}
