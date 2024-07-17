use actix_web::{web, App, HttpResponse, HttpServer};
use std::env;
use dotenv::dotenv;
use nats::asynk::Connection;


extern crate stock_broker_application;

use stock_broker_application::handlers::place_trade::create_trade_request;
use stock_broker_application::handlers::historical_data::get_historical_data;
use stock_broker_application::services::nats_service::connect_to_nats;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let nats_url = env::var("NATS_URL").expect("NATS_URL must be set");
    let nats = connect_to_nats(&nats_url).await.expect("Failed to connect to NATS");

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port.parse().expect("PORT must be a number");

    HttpServer::new(move || {
        App::new()
            .data(nats.clone())
            .route("/trade", web::post().to(create_trade_request))
            .route("/historical_data", web::get().to(get_historical_data))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
