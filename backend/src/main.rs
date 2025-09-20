use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;

mod kafka;
mod opensearch;
mod ml;
mod websocket;
mod api;
mod log_humanizer;
mod gemma_ai;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    // Initialize Gemma AI
    if let Err(e) = gemma_ai::initialize_gemma().await {
        eprintln!("Gemma AI initialization failed: {}", e);
    }
    
    // Start Kafka consumer (skip if Kafka unavailable)
    tokio::spawn(async {
        if let Err(e) = kafka::start_consumer().await {
            eprintln!("Kafka unavailable, running without streaming: {}", e);
        }
    });
    
    // Start ML anomaly detection
    tokio::spawn(ml::start_anomaly_detector());
    
    println!("Mon-X Backend starting on http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
            
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .route("/ws", web::get().to(websocket::websocket_handler))
            .service(
                web::scope("/api")
                    .route("/logs", web::get().to(api::get_logs))
                    .route("/metrics", web::get().to(api::get_metrics))
                    .route("/anomalies", web::get().to(api::get_anomalies))
                    .route("/agents", web::get().to(api::get_agents))
                    .route("/agents/register", web::post().to(api::register_agent))
                    .route("/agents/metrics", web::post().to(api::receive_agent_metrics))
                    .route("/agents/logs", web::post().to(api::receive_agent_logs))
                    .route("/agents/traces", web::post().to(api::receive_agent_traces))
                    .route("/agents/processes", web::post().to(api::receive_agent_processes))
                    .route("/agents/health", web::post().to(api::receive_agent_health))
                    .route("/agents/http-calls", web::post().to(api::receive_http_calls))
                    .route("/agents/database-queries", web::post().to(api::receive_database_queries))
                    .route("/processes", web::get().to(api::get_processes))
                    .route("/health", web::get().to(api::get_health_status))
                    .route("/http-calls", web::get().to(api::get_http_calls))
                    .route("/database-queries", web::get().to(api::get_database_queries))
                    .route("/humanize-log", web::post().to(api::humanize_log))
                    .route("/ai-insights", web::get().to(api::get_ai_insights))
                    .route("/ingest/{topic}", web::post().to(api::ingest_data))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
