package services

import (
    "context"
    "encoding/json"
    "fmt"
    "github.com/nats-io/nats.go"
    "google.golang.org/grpc"
    "log"
    "stock_broker/models"
    pb "stock_broker/proto"
)

func RunHistoricalDataWorker(nc *nats.Conn) {
    sub, err := nc.Subscribe("historical_data_request", func(msg *nats.Msg) {
        var request models.HistoricalDataRequestMessage
        if err := json.Unmarshal(msg.Data, &request); err != nil {
            log.Printf("Failed to deserialize message: %v", err)
            return
        }

       

 log.Printf("Received historical data request: %+v", request)

        response, err := fetchHistoricalData(request)
        if err != nil {
            log.Printf("Failed to fetch historical data: %v", err)
        } else {
            log.Printf("Fetched historical data: %+v", response)
        }
    })
    if err != nil {
        log.Fatalf("Failed to subscribe to 'historical_data_request': %v", err)
    }

    log.Println("Listening for historical data request messages...")
    sub.AutoUnsubscribe(1)
    select {}
}

func fetchHistoricalData(request models.HistoricalDataRequestMessage) (*pb.HistoricalDataResponse, error) {
    conn, err := grpc.Dial("localhost:50051", grpc.WithInsecure())
    if err != nil {
        log.Fatalf("Failed to connect to gRPC server: %v", err)
    }
    defer conn.Close()

    client := pb.NewHistoricalDataServiceClient(conn)

    grpcRequest := &pb.HistoricalDataRequest{
        Symbol: request.Symbol,
        StartTimestamp: request.StartTimestamp,
        EndTimestamp: request.EndTimestamp,
    }

    response, err := client.GetHistoricalData(context.Background(), grpcRequest)
    if err != nil {
        return nil, fmt.Errorf("error while calling GetHistoricalData RPC: %v", err)
    }

    return response, nil
}