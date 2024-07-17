// src/lib.rs

pub mod models;
pub mod handlers {
    pub mod place_trade;
    pub mod historical_data;
}
pub mod services {
    pub mod place_trade_service;
    pub mod historical_data_service;
    pub mod nats_service;
}
