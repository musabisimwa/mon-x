use actix::{Actor, StreamHandler, AsyncContext, ActorContext};
use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use serde_json::json;

pub struct WebSocketSession;

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;
    
    fn started(&mut self, ctx: &mut Self::Context) {
        // Send periodic updates
        ctx.run_interval(std::time::Duration::from_secs(5), |_act, ctx| {
            let anomalies = crate::ml::get_anomalies();
            let message = json!({
                "type": "anomalies",
                "data": anomalies
            });
            ctx.text(message.to_string());
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                // Handle client messages
                ctx.text(text);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

pub async fn websocket_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WebSocketSession, &req, stream)
}
