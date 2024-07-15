use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::models::HistoricalDataRequestMessage;
use crate::services::nats_service::publish_to_nats;
use nats::asynk::Connection;

pub async fn get_historical_data(nats: web::Data<Connection>, query: web::Query<HistoricalDataRequestMessage>) -> impl Responder {
    let message_json = match serde_json::to_string(&query.into_inner()) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Serialization error: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to serialize historical data request"
            }));
        }
    };

    match publish_to_nats(&nats, "historical_data_request", message_json.as_bytes()).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Request for historical data published to NATS"
        })),
        Err(e) => {
            eprintln!("NATS publish error: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to publish request to NATS"
            }))
        }
    }
}
