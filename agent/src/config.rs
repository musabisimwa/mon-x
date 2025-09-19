use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub agent: AgentConfig,
    pub collection: CollectionConfig,
    pub log_paths: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AgentConfig {
    pub name: String,
    pub server_url: String,
    pub report_interval: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CollectionConfig {
    pub metrics: bool,
    pub logs: bool,
    pub resources: bool,
}

pub async fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = tokio::fs::read_to_string(path).await?;
    Ok(serde_yaml::from_str(&content)?)
}
