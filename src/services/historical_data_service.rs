use futures::stream::StreamExt;
use nats::asynk::Connection;
use models::HistoricalDataRequestMessage;
use serde_json::json;

pub async fn run_historical_data_worker(nats: Connection) {
    let subscription = match nats.subscribe("historical_data_request").await {
        Ok(sub) => sub,
        Err(e) => {
            eprintln!("Failed to subscribe to 'historical_data_request': {}", e);
            return;
        }
    };

    let mut messages = subscription.messages();
    println!("Listening for historical data request messages...");

    while let Some(msg) = messages.next().await {
        match msg {
            Ok(msg) => {
                let message_body = msg.data.clone();
                let request: HistoricalDataRequestMessage = match serde_json::from_slice(&message_body) {
                    Ok(request) => request,
                    Err(e) => {
                        eprintln!("Failed to deserialize message: {}", e);
                        continue;
                    }
                };

                // Log the received request
                println!("Received historical data request: {:?}", request);

                // Simulate processing the historical data request
                match process_historical_data_request(request).await {
                    Ok(response) => println!("Processed historical data request: {:?}", response),
                    Err(e) => eprintln!("Failed to process historical data request: {}", e),
                }
            }
            Err(e) => {
                eprintln!("Error while receiving message: {}", e);
            }
        }
    }
}

async fn process_historical_data_request(request: HistoricalDataRequestMessage) -> Result<serde_json::Value, String> {
    // Simulate fetching historical data (replace with actual logic as needed)
    let simulated_response = json!({
        "symbol": request.symbol,
        "data": [
            {"timestamp": request.start_timestamp, "value": 150.0},
            {"timestamp": request.end_timestamp, "value": 155.0}
        ]
    });

    // Simulate a delay for processing
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Here, you can replace this with the actual logic to fetch and process historical data
    Ok(simulated_response)
}