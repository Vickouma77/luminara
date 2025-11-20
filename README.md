# Luminara Bank

A modern, scalable microservices-based banking platform built with Rust.

## 🏗️ Architecture

Luminara follows a microservices architecture with the following services:

### Core Services
- **Auth Service** (Port 8001) - Authentication and authorization
- **User Service** (Port 8002) - User management and profiles
- **Account Service** (Port 8003) - Bank account management
- **Transaction Service** (Port 8004) - Transaction processing and history
- **KYC Service** (Port 8005) - Know Your Customer verification
- **Notification Service** (Port 8006) - Email, SMS, push notifications
- **Payment Service** (Port 8007) - External payment integrations

### Infrastructure
- **API Gateway** (Port 8000) - Single entry point for all services
- **Event Bus** - Async event-driven communication between services

### Shared Libraries
- **shared-domain** - Common domain models and business rules
- **shared-infrastructure** - Configuration and HTTP utilities
- **shared-middleware** - Auth, CORS, logging middleware
- **shared-errors** - Standardized error handling
- **shared-events** - Event definitions for event-driven architecture
- **shared-database** - Database connection pooling and migrations
- **shared-telemetry** - Observability and metrics

## 📋 Prerequisites

- Rust 1.75+ 
- Docker & Docker Compose
- PostgreSQL (via Docker)
- Redis (via Docker)
- Kafka (optional, via Docker)

## 🚀 Quick Start

1. **Clone the repository**
```bash
git clone https://github.com/Vickouma77/luminara.git
cd luminara
```

2. **Set up environment variables**
```bash
cp .env.example .env
# Edit .env with your configuration
```

3. **Start infrastructure services**
```bash
chmod +x scripts/*.sh
./scripts/start-infra.sh
```

4. **Build all services**
```bash
./scripts/build-all.sh
```

5. **Run a specific service**
```bash
cargo run -p auth-service
cargo run -p user-service
# etc.
```

## 🏛️ Project Structure

```
luminara/
├── services/              # Microservices
│   ├── auth-service/
│   ├── user-service/
│   ├── account-service/
│   ├── transaction-service/
│   ├── kyc-service/
│   ├── notification-service/
│   └── payment-service/
├── infrastructure/        # Infrastructure components
│   ├── api-gateway/
│   └── event-bus/
├── shared/               # Shared libraries
│   ├── domain/
│   ├── infrastructure/
│   ├── middleware/
│   ├── errors/
│   ├── events/
│   ├── database/
│   └── telemetry/
├── design/               # Architecture diagrams
├── docs/                 # Documentation
├── docker/               # Docker configurations
├── k8s/                  # Kubernetes manifests
├── scripts/              # Utility scripts
└── Cargo.toml           # Workspace configuration
```

## 🔧 Development

### Running Tests
```bash
cargo test --workspace
```

### Running a Specific Service
```bash
cargo run -p service-name
```

### Database Migrations
```bash
cd services/auth-service
sqlx migrate run
```

## 📊 Observability

- **Jaeger** (Distributed Tracing): http://localhost:16686
- **Prometheus** (Metrics): http://localhost:9090
- **Grafana** (Dashboards): http://localhost:3000 (admin/admin)

## 🛠️ Technology Stack

- **Language**: Rust
- **Web Framework**: Actix-web
- **Database**: PostgreSQL with SQLx
- **Cache**: Redis
- **Message Queue**: Kafka (optional)
- **Observability**: OpenTelemetry, Jaeger, Prometheus, Grafana
- **API**: REST (gRPC planned)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License.

## 👥 Authors

- Victor Ouma - [@Vickouma77](https://github.com/Vickouma77)

## 🗺️ Roadmap

- [x] Initial project structure
- [x] Shared libraries setup
- [ ] Implement Auth Service
- [ ] Implement User Service
- [ ] Implement Account Service
- [ ] Implement Transaction Service
- [ ] Implement KYC Service
- [ ] API Gateway implementation
- [ ] Event-driven architecture with Kafka
- [ ] gRPC support
- [ ] Kubernetes deployment
- [ ] CI/CD pipeline
- [ ] API documentation (OpenAPI/Swagger)