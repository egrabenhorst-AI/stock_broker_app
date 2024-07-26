use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlaceTradeRequest {
    pub symbol: String,
    pub quantity: u32,
    pub action: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoricalDataRequestMessage {
    pub symbol: String,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
}