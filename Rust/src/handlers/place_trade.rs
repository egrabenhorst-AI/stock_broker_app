pub async fn create_trade_request(
    nats: web::Data<Connection>,
    trade_request: web::Json<PlaceTradeRequest>,
) -> HttpResponse {
    let message_json = match serde_json::to_string(&trade_request.into_inner()) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Serialization error: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to serialize trade request"
            }));
        }
    };

    match publish_to_nats(&nats, "place_trade_request", message_json.as_bytes()).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Trade request published to NATS"
        })),
        Err(e) => {
            eprintln!("NATS publish error: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to publish trade request to NATS"
            }))
        }
    }
}
