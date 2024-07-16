use dotenv::dotenv;
use std::env;
use nats::asynk::Connection;

pub mod services {
    pub mod historical_data_service;
    pub mod nats_service;
}

use services::nats_service::connect_to_nats;
use services::historical_data_service::run_historical_data_worker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let nats_url = env::var("NATS_URL").expect("NATS_URL must be set");
    let nats = connect_to_nats(&nats_url).await?;

    run_historical_data_worker(nats).await?;

    Ok(())
}