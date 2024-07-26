package main

import (
    "github.com/gin-gonic/gin"
    "github.com/joho/godotenv"
    "github.com/nats-io/nats.go"
    "log"
    "os"
    "stock_broker/handlers"
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

    port := os.Getenv("PORT")
    if port == "" {
        port = "8080"
    }

    router := gin.Default()
    router.POST("/trade", handlers.CreateTradeRequest(nc))
    router.GET("/historical_data", handlers.GetHistoricalData(nc))

    router.Run(":" + port)
}













































