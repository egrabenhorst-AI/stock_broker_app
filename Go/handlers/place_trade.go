package handlers

import (
    "github.com/gin-gonic/gin"
    "github.com/nats-io/nats.go"
    "net/http"
    "stock_broker/models"
    "stock_broker/services"
)

func CreateTradeRequest(nc *nats.Conn) gin.HandlerFunc {
    return func(c *gin.Context) {
        var tradeRequest models.PlaceTradeRequest
        if err := c.ShouldBindJSON(&tradeRequest); err != nil {
            c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid request"})
            return
        }

        message, err := json.Marshal(tradeRequest)
        if err != nil {
            c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to serialize trade request"})
            return
        }

        err = services.PublishToNATS(nc, "place_trade_request", message)
        if err != nil {
            c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to publish trade request to NATS"})
            return
        }

        c.JSON(http.StatusOK, gin.H{"status": "success", "message": "Trade request published to NATS"})
    }
}