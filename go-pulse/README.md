# go-pulse

This Go server creates short-lived goroutines that communicate with a Rust server.

## 🌟 Features

- 🔢 Creates a random number of goroutines (1-10)
- ⏱️ Each goroutine lives for 10-15 seconds
- 🔁 Continuously sends data to Rust server
- 🆔 Uses unique identifiers for each goroutine

## 🛠️ Dependencies

- 🍹 Gin web framework

## 🚀 How to Run

1. Create .env file:
```bash
cp .env.example .env
```

2. Run Server:
```bash
make dev
```
3. 🌐 The server will start on `http://localhost:8000`

## Endpoints

- GET `/trigger`: Triggers the creation of goroutines

## 📊 How it works

1. 🎬 When `/trigger` is hit, it creates 1-10 goroutines
2. 🕐 Each goroutine lives for 10-15 seconds
3. 📡 Goroutines repeatedly send data to the Rust server
4. 📦 Data includes a unique identifier and timestamp

## ⚠️ Note

Ensure the Rust server and Redis are running before starting this server.
