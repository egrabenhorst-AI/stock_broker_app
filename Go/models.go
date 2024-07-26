package models

type PlaceTradeRequest struct {
    Symbol   string `json:"symbol"`
    Quantity uint32 `json:"quantity"`
    Action   string `json:"action"`
}

type HistoricalDataRequestMessage struct {
    Symbol        string `json:"symbol"`
    StartTimestamp int64  `json:"start_timestamp"`
    EndTimestamp   int64  `json:"end_timestamp"`
}