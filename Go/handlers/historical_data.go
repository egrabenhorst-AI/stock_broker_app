package handlers

import (
    "github.com/gin-gonic/gin"
    "github.com/nats-io/nats.go"
    "net/http"
    "stock_broker/models"
    "stock_broker/services"
)

func GetHistoricalData(nc *nats.Conn) gin.HandlerFunc {
    return func(c *gin.Context) {
        var query models.HistoricalDataRequestMessage
        if err := c.ShouldBindQuery(&query); err != nil {
            c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid request"})
            return
        }

        message, err := json.Marshal(query)
        if err != nil {
            c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to serialize historical data request"})
            return
        }

        err = services.PublishToNATS(nc, "historical_data_request", message)
        if err != nil {
            c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to publish request to NATS"})
            return
        }

        c.JSON(http.StatusOK, gin.H{"status": "success", "message": "Request for historical data published to NATS"})
    }
}