use std::time::Duration;
use tokio::time;

mod config;
mod collector;
mod reporter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load_config("config.yaml").await?;
    println!("Starting comprehensive monitoring agent: {}", config.agent.name);
    
    // Register with server
    reporter::register_agent(&config).await?;
    
    let mut interval = time::interval(Duration::from_secs(config.agent.report_interval));
    
    loop {
        interval.tick().await;
        
        // Collect all data types
        let metrics = collector::collect_metrics(&config).await;
        let logs = collector::collect_logs(&config).await;
        let traces = collector::collect_traces(&config).await;
        let processes = collector::collect_processes(&config).await;
        let health_data = collector::collect_health_checks(&config).await;
        let http_calls = collector::collect_http_calls(&config).await;
        let db_queries = collector::collect_database_queries(&config).await;
        
        // Capture lengths before moving data
        let metrics_len = metrics.len();
        let logs_len = logs.len();
        let traces_len = traces.len();
        let processes_len = processes.len();
        let health_len = health_data.len();
        let http_len = http_calls.len();
        let db_len = db_queries.len();
        
        // Send to server
        reporter::send_comprehensive_data(&config, metrics, logs, traces, processes, health_data).await?;
        reporter::send_http_calls(&config, http_calls).await?;
        reporter::send_database_queries(&config, db_queries).await?;
        
        println!("Sent: {} metrics, {} logs, {} traces, {} processes, {} health, {} http, {} db", 
                metrics_len, logs_len, traces_len, processes_len, health_len, http_len, db_len);
    }
}
