package services

import (
    "github.com/nats-io/nats.go"
    "log"
)

func ConnectToNATS(url string) (*nats.Conn, error) {
    nc, err := nats.Connect(url)
    if err != nil {
        return nil, err
    }

    return nc, nil
}

func PublishToNATS(nc *nats.Conn, subject string, message []byte) error {
    err := nc.Publish(subject, message)
    if err != nil {
        log.Printf("Failed to publish to NATS: %v", err)
        return err
    }

    return nil
}