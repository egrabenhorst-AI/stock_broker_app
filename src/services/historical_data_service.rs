use futures::stream::StreamExt;
use nats::asynk::Connection;
use models::HistoricalDataRequestMessage;
use serde_json::json;
use tonic::{transport::Channel, Request};
use async_trait::async_trait;
use crate::services::historical_data::historicaldata::{
    historical_data_service_client::HistoricalDataServiceClient,
    HistoricalDataRequest,
    HistoricalDataEntry,
    HistoricalDataResponse
};

pub mod historicaldata {
    tonic::include_proto!("historicaldata");
}

// Function to run the historical data worker
pub async fn run_historical_data_worker(nats: Connection) -> Result<(), Box<dyn std::error::Error>> {
    let subscription = match nats.subscribe("historical_data_request").await {
        Ok(sub) => sub,
        Err(e) => {
            eprintln!("Failed to subscribe to 'historical_data_request': {}", e);
            return Ok(());
        }
    };

    let mut messages = subscription.messages();
    println!("Listening for historical data request messages...");

    // Connect to the gRPC server
    let channel = Channel::from_static("http://[::1]:50051").connect().await?;
    let mut client = HistoricalDataServiceClient::new(channel);

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

                // Process the historical data request by making a gRPC call
                match process_historical_data_request(&mut client, request).await {
                    Ok(response) => println!("Processed historical data request: {:?}", response),
                    Err(e) => eprintln!("Failed to process historical data request: {}", e),
                }
            }
            Err(e) => {
                eprintln!("Error while receiving message: {}", e);
            }
        }
    }

    Ok(())
}

// Function to process historical data requests by making a gRPC call
async fn process_historical_data_request(client: &mut HistoricalDataServiceClient<Channel>, request: HistoricalDataRequestMessage) -> Result<serde_json::Value, String> {
    let grpc_request = HistoricalDataRequest {
        symbol: request.symbol,
        start_date: request.start_timestamp,
        end_date: request.end_timestamp,
    };

    // Make the gRPC call to fetch historical data
    let response: HistoricalDataResponse = match client.get_historical_data(Request::new(grpc_request)).await {
        Ok(response) => response.into_inner(),
        Err(e) => return Err(format!("gRPC request failed: {}", e)),
    };

    // Convert the response to JSON format
    let response_json = json!({
        "symbol": request.symbol,
        "data": response.entries.iter().map(|entry| json!({"timestamp": entry.date, "value": entry.value})).collect::<Vec<_>>()
    });

    Ok(response_json)
}
