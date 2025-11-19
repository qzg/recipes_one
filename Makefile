.PHONY: help build run test clean docker-up docker-down migrate

help:
	@echo "Available commands:"
	@echo "  make build        - Build all workspace crates"
	@echo "  make run          - Run the server locally"
	@echo "  make test         - Run all tests"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make docker-up    - Start all services with Docker"
	@echo "  make docker-down  - Stop all Docker services"
	@echo "  make migrate      - Run database migrations"
	@echo "  make fmt          - Format code"
	@echo "  make clippy       - Run clippy lints"

build:
	cargo build --all

run:
	cargo run -p server

test:
	cargo test --all

clean:
	cargo clean

docker-up:
	docker-compose up -d
	@echo "Services started! Backend: http://localhost:3000"

docker-down:
	docker-compose down

docker-logs:
	docker-compose logs -f

migrate:
	cd crates/server && sqlx migrate run

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets -- -D warnings

dev:
	@echo "Starting development environment..."
	docker-compose up -d postgres
	@echo "Waiting for PostgreSQL to be ready..."
	@sleep 3
	$(MAKE) migrate
	@echo "Starting server..."
	cargo run -p server
