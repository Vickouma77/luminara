# Makefile for Luminara Banking Platform

.PHONY: help build test run clean docker-up docker-down fmt lint

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-20s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Build all services
	@echo "Building all services..."
	@cargo build --workspace

build-release: ## Build all services in release mode
	@echo "Building all services in release mode..."
	@cargo build --workspace --release

test: ## Run all tests
	@echo "Running tests..."
	@cargo test --workspace

test-coverage: ## Run tests with coverage
	@echo "Running tests with coverage..."
	@cargo tarpaulin --workspace --out Html

run-auth: ## Run auth service
	@cargo run -p auth-service

run-user: ## Run user service
	@cargo run -p user-service

run-account: ## Run account service
	@cargo run -p account-service

run-gateway: ## Run API gateway
	@cargo run -p api-gateway

clean: ## Clean build artifacts
	@echo "Cleaning..."
	@cargo clean

docker-up: ## Start all infrastructure services
	@echo "Starting infrastructure services..."
	@docker-compose up -d
	@echo "Waiting for services to be ready..."
	@sleep 10

docker-down: ## Stop all infrastructure services
	@echo "Stopping infrastructure services..."
	@docker-compose down

docker-logs: ## Show docker logs
	@docker-compose logs -f

fmt: ## Format code
	@echo "Formatting code..."
	@cargo fmt --all

lint: ## Run clippy
	@echo "Running clippy..."
	@cargo clippy --workspace --all-targets -- -D warnings

check: ## Check code without building
	@cargo check --workspace

watch: ## Watch and rebuild on changes
	@cargo watch -x 'build --workspace'

setup: ## Initial setup
	@echo "Setting up development environment..."
	@cp .env.example .env
	@chmod +x scripts/*.sh
	@echo "Setup complete! Edit .env file with your configuration."

db-migrate: ## Run database migrations for all services
	@echo "Running migrations..."
	@cd services/auth-service && sqlx migrate run
	@cd services/user-service && sqlx migrate run
	@cd services/account-service && sqlx migrate run

docs: ## Generate documentation
	@cargo doc --workspace --no-deps --open

audit: ## Security audit
	@cargo audit

update: ## Update dependencies
	@cargo update

all: fmt lint test build ## Run fmt, lint, test, and build
