# rust-pulse

This Rust server receives data from Go goroutines and stores it in Redis.

## 🌟 Features

- 🌐 Built with Actix web framework
- 🔴 Integrates with Redis for data storage
- 🔄 Asynchronous handling of requests
- 📊 Stores goroutine data as lists in Redis

## 🛠️ Dependencies

- 🌐 actix-web
- 🔴 redis
- 🕰️ tokio
- 📦 serde

## How to run:

1. Build and run the server: 
```bash
cargo run
```

2. 🌐 The server will start on http://localhost:8001

## 🔍 Endpoints

**GET /trigger**: Endpoint to trigger Go server
**POST /receive**: Receives data from Go goroutines

## 📊 How it works

📥 Receives data from Go goroutines via /receive  
🔢 Each request contains a goroutine identifier and timestamp  
🔴 Stores the data in Redis, using the identifier as key and timestamps as a list  

## ⚠️ Note
Ensure Redis is running before starting this server.
