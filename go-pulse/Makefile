.PHONY: dev build run test

dev:
	@echo "Running the application"
	@go run cmd/app/main.go

build:
	@echo "Building the application"
	@go build -o build/go-pulse cmd/app/main.go

run:
	@echo "Running the application"
	@./build/go-pulse
