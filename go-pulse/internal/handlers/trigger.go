package handlers

import (
	"bytes"
	"encoding/json"
	"fmt"
	"math/rand"
	"net/http"
	"sync"
	"time"

	"github.com/benodiwal/go-pulse/internal/env"
	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
)

func TriggerHandler(ctx *gin.Context) {
	numOfGoRoutines := rand.Intn(10) + 1
	var wg sync.WaitGroup

	for i := 0; i < numOfGoRoutines; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			identifier := uuid.New().String()

			duration := time.Duration(10+rand.Intn(6)) * time.Second
            endTime := time.Now().Add(duration)

			for time.Now().Before(endTime) {
                sendData(identifier)
                time.Sleep(100 * time.Millisecond)
            }
		}()
	}

	wg.Wait()
	ctx.JSON(http.StatusOK, gin.H { "message": "Goroutines created and finished" })
}

func sendData(identifier string) {
    timestamp := time.Now().UTC().Format(time.RFC3339)
    payload := map[string]string{
        "identifier": identifier,
        "timestamp":  timestamp,
    }

    jsonPayload, err := json.Marshal(payload)
    if err != nil {
        fmt.Printf("Error marshaling JSON: %v\n", err)
        return
    }

	endpoint := fmt.Sprintf("%s/receive", env.Read(env.RUST_PULSE_ENDPOINT))
    resp, err := http.Post(endpoint, "application/json", bytes.NewBuffer(jsonPayload))
    if err != nil {
        fmt.Printf("Error sending data to Rust Pulse: %v\n", err)
        return
    }
    defer resp.Body.Close()

    fmt.Printf("Sent data to Rust Pulse: %v, Response: %v\n", payload, resp.Status)
}
