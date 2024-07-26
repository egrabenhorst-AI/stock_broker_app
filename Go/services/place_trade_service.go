package services

import (
    "encoding/json"
    "fmt"
    "github.com/nats-io/nats.go"
    "log"
    "stock_broker/models"
    "time"
)

func RunPlaceTradeRequestWorker(nc *nats.Conn) {
    sub, err := nc.Subscribe("place_trade_request", func(msg *nats.Msg) {
        var tradeRequest models.PlaceTradeRequest
        if err := json.Unmarshal(msg.Data, &tradeRequest); err != nil {
            log.Printf("Failed to deserialize message: %v", err)
            return
        }

        log.Printf("Received place trade request: %+v", tradeRequest)

        response, err := processTradeRequest(tradeRequest)
        if err != nil {
            log.Printf("Failed to process trade request: %v", err)
        } else {
            log.Printf("Processed trade request: %+v", response)
        }
    })
    if err != nil {
        log.Fatalf("Failed to subscribe to 'place_trade_request': %v", err)
    }

    log.Println("Listening for place trade request messages...")
    sub.AutoUnsubscribe(1)
    select {}
}

func processTradeRequest(tradeRequest models.PlaceTradeRequest) (map[string]interface{}, error) {
    simulatedResponse := map[string]interface{}{
        "symbol":  tradeRequest.Symbol,
        "quantity": tradeRequest.Quantity,
        "action":  tradeRequest.Action,
        "status":  "success",
        "message": "Trade executed successfully",
    }

    time.Sleep(2 * time.Second)

    return simulatedResponse, nil
}