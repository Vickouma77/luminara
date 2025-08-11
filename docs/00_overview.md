# Luminara — High-Level Overview

## 1. Introduction
Luminara is a Rust-based microservices platform designed for **high performance**, **safety**, and **scalability**.  
The system follows a **service-oriented architecture**, where each service is independently deployable, owns its own data, and communicates via well-defined APIs or asynchronous events.

The goal is to build a modular foundation that allows:
- Fast service development without duplicating boilerplate
- Clear separation of business concerns
- Easy integration with observability, security, and infrastructure tooling

---

## 2. Architecture

### 2.1 Overall Style
- **Architecture type:** Microservices  
- **Pattern:** Hexagonal / Ports & Adapters  
- **Communication:**
  - Internal: gRPC or HTTP (Actix-web)
  - Async messaging: Kafka (optional, for event-driven workflows)
- **Persistence:** Each service owns its own database schema (PostgreSQL in initial setup)

### 2.2 Workspace Structure
The codebase is organized as a Rust **workspace**:


---

## 3. Tech Stack

### 3.1 Core Languages & Frameworks
- **Language:** Rust (edition 2021)
- **Web Framework:** Actix-web
- **Async Runtime:** Tokio

### 3.2 Data Layer
- **Database:** PostgreSQL (SQLx or deadpool-postgres for async DB access)
- **Migrations:** SQLx migrations

### 3.3 Messaging & Events (optional in early phase)
- Kafka via `rdkafka` for async communication

### 3.4 Observability
- **Logging:** `tracing` + `tracing-subscriber`
- **Metrics:** `opentelemetry` + Prometheus
- **Distributed Tracing:** OpenTelemetry collector + Jaeger/Tempo

### 3.5 Configuration & Environment Management
- `config` crate with `.env` + environment variable overrides
- Separate configs for dev, staging, production

### 3.6 Testing
- Unit tests for domain logic
- Integration tests using `tokio::test` + `reqwest`
- Testcontainers for DB/Kafka where needed

---

## 4. Goals

### 4.1 Primary Goals
1. Build a **scalable** and **extensible** microservices foundation
2. Provide a **shared infrastructure layer** to eliminate boilerplate
3. Ensure each service is **independently deployable** and **resilient**
4. Maintain strong **observability** from the start

### 4.2 Non-Goals (Initial Phase)
- Full CI/CD automation (will be added after MVP)
- Multi-region deployments (single-region for MVP)
- Full-blown service mesh (basic service-to-service comms first)

---

## 5. Guiding Principles
- **Small, focused services** — each with a single responsibility
- **API contracts first** — document and version APIs
- **DRY but clear boundaries** — share code via library crates, not by copying
- **Fail fast, recover gracefully** — timeouts, retries, and circuit breakers
- **Observability by default** — logs, metrics, and traces in every service

---

## 6. Roadmap (Initial Phase)
1. **Workspace setup** — root structure, shared crates
2. **Infra crate** — config, logging, DB, middleware
3. **First service (User)** — CRUD endpoints, DB persistence
4. **Second service (Account)** — integrates with User via HTTP
5. **Basic observability stack** — Prometheus, Grafana, Jaeger
6. **Dockerized local dev environment** — DB + services
