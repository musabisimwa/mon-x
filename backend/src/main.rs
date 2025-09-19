use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;

mod kafka;
mod opensearch;
mod ml;
mod websocket;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    // Start Kafka consumer
    tokio::spawn(kafka::start_consumer());
    
    // Start ML anomaly detection
    tokio::spawn(ml::start_anomaly_detector());
    
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
                    .route("/processes", web::get().to(api::get_processes))
                    .route("/health", web::get().to(api::get_health_status))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
