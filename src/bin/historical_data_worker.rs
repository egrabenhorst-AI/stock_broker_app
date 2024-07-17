use dotenv::dotenv;
use std::env;
use nats::asynk::Connection;

extern crate stock_broker_application;

use stock_broker_application::services::nats_service::connect_to_nats;
use stock_broker_application::services::historical_data_service::run_historical_data_worker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let nats_url = env::var("NATS_URL").expect("NATS_URL must be set");
    let nats = connect_to_nats(&nats_url).await?;

    run_historical_data_worker(nats).await?;

    Ok(())
}