# Quick Start Guide

## Prerequisites

Ensure you have the following installed:
- Rust 1.75+: `rustup update`
- Docker & Docker Compose
- Make (optional, for convenience)

## Step-by-Step Setup

### 1. Environment Configuration

```bash
# Copy environment template
cp .env.example .env

# Edit .env with your settings (optional for local dev)
nano .env
```

### 2. Start Infrastructure Services

```bash
# Using script
./scripts/start-infra.sh

# Or using make
make docker-up

# Or directly with docker-compose
docker-compose up -d
```

This starts:
- PostgreSQL (port 5432)
- Redis (port 6379)
- Kafka + Zookeeper (port 9092)
- Jaeger (port 16686)
- Prometheus (port 9090)
- Grafana (port 3000)

### 3. Build the Workspace

```bash
# Using script
./scripts/build-all.sh

# Or using make
make build

# Or directly with cargo
cargo build --workspace
```

### 4. Run Services

**Option A: Run Individual Services**
```bash
# Terminal 1: API Gateway
cargo run -p api-gateway

# Terminal 2: Auth Service
cargo run -p auth-service

# Terminal 3: User Service
cargo run -p user-service

# Add more services as needed...
```

**Option B: Using Make**
```bash
# In separate terminals
make run-gateway
make run-auth
make run-user
```

### 5. Verify Services

Check that services are running:
```bash
# Health checks (once implemented)
curl http://localhost:8000/health  # API Gateway
curl http://localhost:8001/health  # Auth Service
curl http://localhost:8002/health  # User Service
```

### 6. Access Observability Tools

- **Jaeger UI**: http://localhost:16686
- **Prometheus**: http://localhost:9090
- **Grafana**: http://localhost:3000 (admin/admin)

## Common Commands

### Development
```bash
make build              # Build all services
make test               # Run tests
make fmt                # Format code
make lint               # Run clippy
make check              # Check without building
```

### Docker
```bash
make docker-up          # Start infrastructure
make docker-down        # Stop infrastructure
make docker-logs        # View logs
```

### Database
```bash
make db-migrate         # Run migrations
```

## Service Ports

| Service              | Port |
|---------------------|------|
| API Gateway         | 8000 |
| Auth Service        | 8001 |
| User Service        | 8002 |
| Account Service     | 8003 |
| Transaction Service | 8004 |
| KYC Service         | 8005 |
| Notification Service| 8006 |
| Payment Service     | 8007 |

## Development Tips

1. **Watch Mode**: Use `cargo watch` for auto-rebuild
   ```bash
   cargo install cargo-watch
   cargo watch -x 'run -p service-name'
   ```

2. **Specific Service**: Build/test single service
   ```bash
   cargo build -p auth-service
   cargo test -p user-service
   ```

3. **Check Workspace**: Verify all services compile
   ```bash
   cargo check --workspace
   ```

4. **Clean Build**: Remove build artifacts
   ```bash
   cargo clean
   ```

## Troubleshooting

### Port Already in Use
```bash
# Find process using port
lsof -i :8001

# Kill process
kill -9 <PID>
```

### Database Connection Failed
```bash
# Check if PostgreSQL is running
docker ps | grep postgres

# Restart PostgreSQL
docker-compose restart postgres
```

### Build Errors
```bash
# Update dependencies
cargo update

# Clean and rebuild
cargo clean && cargo build --workspace
```

## Next Steps

1. **Read Documentation**
   - `docs/00_overview.md` - High-level architecture
   - `docs/02_improved_organization.md` - Organization details
   - `docs/04_service_development_guide.md` - How to create services
   - `docs/05_architecture_overview.md` - Detailed architecture

2. **Implement Services**
   - Start with Auth Service
   - Then User Service
   - Follow the service development guide

3. **Add Tests**
   - Unit tests for business logic
   - Integration tests for APIs
   - E2E tests for critical flows

4. **Configure Observability**
   - Set up Grafana dashboards
   - Configure alerts in Prometheus
   - Test distributed tracing

## Getting Help

- Check documentation in `docs/` directory
- Review existing service implementations
- Check PlantUML diagrams in `design/` directory

## Stopping Services

```bash
# Stop infrastructure
./scripts/stop-all.sh

# Or
make docker-down

# Stop running services: Ctrl+C in each terminal
```

## Production Deployment

For production deployment:
1. Review `k8s/` directory for Kubernetes manifests
2. Configure proper secrets management
3. Set up CI/CD pipeline
4. Configure monitoring and alerting
5. Implement proper backup strategies