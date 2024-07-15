use dotenv::dotenv;
use std::env;
use nats::asynk::Connection;
use crate::services::nats_service::connect_to_nats;
use crate::services::place_trade_service::run_place_trade_request_worker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let nats_url = env::var("NATS_URL").expect("NATS_URL must be set");
    let nats = connect_to_nats(&nats_url).await?;

    run_place_trade_request_worker(nats).await?;

    Ok(())
}