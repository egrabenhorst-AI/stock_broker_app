use futures::stream::StreamExt;
use nats::{asynk::Connection, Error};
use serde_json::json;
use models::PlaceTradeRequest;

pub async fn run_place_trade_request_worker(nats: Connection) -> Result<(), Error> {
    let subscription = match nats.subscribe("place_trade_request").await {
        Ok(sub) => sub,
        Err(e) => {
            eprintln!("Failed to subscribe to 'place_trade_request': {}", e);
            return Err(e);
        }
    };

    let mut messages = subscription.messages();
    println!("Listening for place trade request messages...");

    while let Some(msg) = messages.next().await {
        match msg {
            Ok(msg) => {
                let message_body = msg.data.clone();
                let trade_request: PlaceTradeRequest = match serde_json::from_slice(&message_body) {
                    Ok(request) => request,
                    Err(e) => {
                        eprintln!("Failed to deserialize message: {}", e);
                        continue;
                    }
                };

                // Log the received trade request
                println!("Received place trade request: {:?}", trade_request);

                // Simulate processing the trade request
                match process_trade_request(trade_request).await {
                    Ok(response) => println!("Processed trade request: {:?}", response),
                    Err(e) => eprintln!("Failed to process trade request: {}", e),
                }
            }
            Err(e) => {
                eprintln!("Error while receiving message: {:?}", e);
            }
        }
    }

    Ok(())
}

async fn process_trade_request(trade_request: PlaceTradeRequest) -> Result<serde_json::Value, String> {
    // Simulate processing the trade request (replace with actual logic as needed)
    let simulated_response = json!({
        "symbol": trade_request.symbol,
        "quantity": trade_request.quantity,
        "action": trade_request.action,
        "status": "success",
        "message": "Trade executed successfully"
    });

    // Simulate a delay for processing
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Here, you can replace this with the actual logic to process the trade request
    Ok(simulated_response)
}