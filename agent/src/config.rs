use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub agent: AgentConfig,
    pub collection: CollectionConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AgentConfig {
    pub name: String,
    pub server_url: String,
    pub report_interval: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CollectionConfig {
    pub metrics: bool,
    pub logs: bool,
    pub traces: bool,
    pub processes: bool,
    pub health: bool,
    pub docker: bool,
    pub nginx_log_path: Option<String>,
    pub health_check_urls: Vec<String>,
}

pub async fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = tokio::fs::read_to_string(path).await?;
    Ok(serde_yaml::from_str(&content)?)
}
