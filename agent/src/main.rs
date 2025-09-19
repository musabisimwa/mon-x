use std::time::Duration;
use tokio::time;

mod config;
mod collector;
mod reporter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load_config("config.yaml").await?;
    println!("Starting agent: {}", config.agent.name);
    
    // Register with server
    reporter::register_agent(&config).await?;
    
    let mut interval = time::interval(Duration::from_secs(config.agent.report_interval));
    
    loop {
        interval.tick().await;
        
        let metrics = collector::collect_metrics(&config).await;
        let logs = collector::collect_logs(&config).await;
        
        reporter::send_data(&config, metrics, logs).await?;
    }
}
