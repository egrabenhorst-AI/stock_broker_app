package main

import (
    "github.com/joho/godotenv"
    "log"
    "os"
    "stock_broker/services"
)

func main() {
    err := godotenv.Load()
    if err != nil {
        log.Fatalf("Error loading .env file")
    }

    natsURL := os.Getenv("NATS_URL")
    nc, err := services.ConnectToNATS(natsURL)
    if err != nil {
        log.Fatalf("Failed to connect to NATS: %v", err)
    }

    services.RunPlaceTradeRequestWorker(nc)
}