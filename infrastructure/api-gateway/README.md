# API Gateway

The API Gateway is the single entry point for all client requests to the Luminara banking platform microservices.

## Table of Contents

- [Features](#features)
- [Architecture](#architecture)
- [Project Structure](#project-structure)
- [Library Usage](#library-usage)
- [API Endpoints](#api-endpoints)
- [Configuration](#configuration)
- [Running the Gateway](#running-the-gateway)
- [Development](#development)
- [Testing](#testing)
- [Security](#security)
- [Troubleshooting](#troubleshooting)

## Features

- **Request Routing**: Routes requests to appropriate microservices
- **Authentication**: JWT-based authentication for protected endpoints
- **Rate Limiting**: Prevents abuse with configurable rate limits (100 req/60s per IP)
- **Request Forwarding**: Proxies requests to backend services with connection pooling
- **CORS**: Cross-Origin Resource Sharing support
- **Logging**: Structured JSON logging with tracing
- **Library + Binary**: Can be used as both a standalone service and a reusable library

## Architecture

```
Client Request
     ↓
API Gateway (Port 8000)
     ├── Rate Limiting
     ├── Authentication (for protected routes)
     ├── Request Routing
     └── Proxy to Services
           ├── Auth Service (8001)
           ├── User Service (8002)
           ├── Account Service (8003)
           ├── Transaction Service (8004)
           ├── KYC Service (8005)
           ├── Payment Service (8007)
           └── Notification Service (8006)
```

## Project Structure

```
api-gateway/
├── Cargo.toml                    # [lib] + [[bin]] configuration
└── src/
    ├── lib.rs                    # Library entry point (public API)
    ├── main.rs                   # Binary entry point
    ├── config/
    │   └── mod.rs               # Configuration management
    ├── routes/
    │   └── mod.rs               # Route definitions and handlers
    ├── proxy/
    │   └── mod.rs               # HTTP client and request forwarding
    └── middleware/
        ├── mod.rs               # Middleware module exports
        ├── auth/
        │   └── mod.rs           # JWT authentication middleware
        └── rate_limit/
            └── mod.rs           # Rate limiting middleware
```

### Cargo.toml Configuration

The API Gateway is configured as both a library and binary:

```toml
[lib]
name = "api_gateway"
path = "src/lib.rs"

[[bin]]
name = "api-gateway"
path = "src/main.rs"
```

## Library Usage

The API Gateway can be used as a library in other services:

### Add as Dependency

```toml
[dependencies]
api-gateway = { path = "../../infrastructure/api-gateway" }
```

### Import and Use

```rust
// Use the prelude for convenient imports
use api_gateway::prelude::*;

// Access configuration
let config = Config::load()?;

// Use middleware components
let auth = AuthMiddleware;
let limiter = RateLimiter::new(100, 60);

// Import specific modules
use api_gateway::config::ServerConfig;
use api_gateway::middleware::auth::Claims;
use api_gateway::proxy::forward_request;
```

### Generate Documentation

```bash
cargo doc -p api-gateway --no-deps --open
```

## API Endpoints

### Public Endpoints (No Authentication Required)

#### Health Check
- `GET /api/v1/health` - Health check endpoint

#### Authentication
- `POST /api/v1/auth/login` - User login
- `POST /api/v1/auth/register` - User registration
- `POST /api/v1/auth/refresh` - Refresh access token
- `POST /api/v1/auth/logout` - User logout
- `GET /api/v1/auth/verify` - Verify token

### Protected Endpoints (Authentication Required)

All protected endpoints require an `Authorization` header with a valid JWT token:
```
Authorization: Bearer <your-jwt-token>
```

#### User Management
- `GET /api/v1/users` - List users
- `POST /api/v1/users` - Create user
- `GET /api/v1/users/{id}` - Get user by ID
- `PUT /api/v1/users/{id}` - Update user
- `DELETE /api/v1/users/{id}` - Delete user
- `GET /api/v1/users/{id}/profile` - Get user profile
- `PUT /api/v1/users/{id}/profile` - Update user profile

#### Account Management
- `GET /api/v1/accounts` - List accounts
- `POST /api/v1/accounts` - Create account
- `GET /api/v1/accounts/{id}` - Get account by ID
- `PUT /api/v1/accounts/{id}` - Update account
- `DELETE /api/v1/accounts/{id}` - Close account
- `GET /api/v1/accounts/{id}/balance` - Get account balance
- `GET /api/v1/accounts/{id}/transactions` - Get account transactions

#### Transactions
- `GET /api/v1/transactions` - List transactions
- `POST /api/v1/transactions` - Create transaction
- `GET /api/v1/transactions/{id}` - Get transaction by ID
- `GET /api/v1/transactions/{id}/status` - Get transaction status
- `POST /api/v1/transactions/transfer` - Transfer money
- `POST /api/v1/transactions/withdraw` - Withdraw money
- `POST /api/v1/transactions/deposit` - Deposit money

#### KYC
- `POST /api/v1/kyc` - Submit KYC
- `GET /api/v1/kyc/{id}` - Get KYC status
- `POST /api/v1/kyc/{id}/documents` - Upload KYC documents
- `GET /api/v1/kyc/{id}/status` - Check KYC status
- `POST /api/v1/kyc/{id}/verify` - Verify KYC

#### Payments
- `GET /api/v1/payments` - List payments
- `POST /api/v1/payments` - Create payment
- `GET /api/v1/payments/{id}` - Get payment by ID
- `GET /api/v1/payments/{id}/status` - Get payment status
- `POST /api/v1/payments/{id}/refund` - Refund payment
- `POST /api/v1/payments/process` - Process payment

## Configuration

The API Gateway can be configured via environment variables:

```bash
# Server Configuration
API_GATEWAY__SERVER__HOST=0.0.0.0
API_GATEWAY__SERVER__PORT=8000

# Service URLs
API_GATEWAY__SERVICES__AUTH_SERVICE_URL=http://localhost:8001
API_GATEWAY__SERVICES__USER_SERVICE_URL=http://localhost:8002
API_GATEWAY__SERVICES__ACCOUNT_SERVICE_URL=http://localhost:8003
API_GATEWAY__SERVICES__TRANSACTION_SERVICE_URL=http://localhost:8004
API_GATEWAY__SERVICES__KYC_SERVICE_URL=http://localhost:8005
API_GATEWAY__SERVICES__PAYMENT_SERVICE_URL=http://localhost:8007
API_GATEWAY__SERVICES__NOTIFICATION_SERVICE_URL=http://localhost:8006

# Authentication
JWT_SECRET=your-secret-key-change-in-production
```

## Rate Limiting

The API Gateway implements rate limiting to prevent abuse:
- **Default**: 100 requests per 60 seconds per IP address
- Rate limit is applied globally to all endpoints
- Returns `429 Too Many Requests` when limit is exceeded

## Authentication Middleware

Protected endpoints use JWT-based authentication:
1. Client includes `Authorization: Bearer <token>` header
2. Gateway validates the JWT token
3. If valid, request is forwarded to the backend service
4. If invalid, returns `401 Unauthorized`

The JWT token must include:
- `sub`: User ID
- `exp`: Expiration timestamp
- `iat`: Issued at timestamp
- `email`: User email

## Request Flow

1. **Client sends request** to API Gateway
2. **Rate limiting** checks request count
3. **Authentication** validates JWT token (if protected endpoint)
4. **Routing** determines target service
5. **Proxy** forwards request to backend service
6. **Response** returns backend response to client

## Running the Gateway

### Development
```bash
# Run the gateway
cargo run -p api-gateway

# With custom logging
RUST_LOG=debug cargo run -p api-gateway
```

### Build Options

```bash
# Build library only
cargo build -p api-gateway --lib

# Build binary only
cargo build -p api-gateway --bin api-gateway

# Build both (default)
cargo build -p api-gateway

# Release build
cargo build --release -p api-gateway
./target/release/api-gateway
```

### With Docker
```bash
docker build -f docker/api-gateway.Dockerfile -t luminara/api-gateway .
docker run -p 8000:8000 luminara/api-gateway
```

## Logging

The API Gateway uses structured logging with the `tracing` crate:
- Request/response logging
- Authentication events
- Rate limiting events
- Proxy forwarding logs
- Error logs

View logs in JSON format for easy parsing and analysis.

## Error Handling

The API Gateway handles various error scenarios:

- **400 Bad Request**: Invalid request format
- **401 Unauthorized**: Missing or invalid authentication
- **404 Not Found**: Endpoint not found
- **429 Too Many Requests**: Rate limit exceeded
- **502 Bad Gateway**: Backend service unavailable
- **503 Service Unavailable**: Gateway overloaded

## Monitoring

The API Gateway exposes metrics for monitoring:
- Request count by endpoint
- Request latency
- Error rates
- Rate limit hits
- Backend service health

## Security

- **HTTPS**: Use HTTPS in production
- **JWT Validation**: All protected endpoints validate JWT tokens
- **Rate Limiting**: Prevents DoS attacks
- **CORS**: Configured for allowed origins
- **Header Forwarding**: Sanitizes headers before forwarding
- **Secret Management**: Use environment variables for secrets

## Development

### Module Organization

The gateway follows proper Rust module conventions:

- Each module is in its own directory with `mod.rs`
- `lib.rs` exposes public API for library usage
- `main.rs` uses the library to run the binary
- Easy to extend by adding submodules

### Adding a New Route

1. Add route configuration in `src/routes/mod.rs`
2. Add proxy handler if needed
3. Apply `AuthMiddleware` if the route is protected
4. Update this README

### Adding New Features

```rust
// Example: Adding a new middleware
// 1. Create src/middleware/logging/mod.rs
// 2. Export in src/middleware/mod.rs
pub mod logging;

// 3. Add to lib.rs
pub use middleware::logging::LoggingMiddleware;
```

## Testing

### Manual Testing

```bash
# Health check
curl http://localhost:8000/api/v1/health

# Login (when auth service is running)
curl -X POST http://localhost:8000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"password"}'

# Protected endpoint with token
export TOKEN="your-jwt-token"
curl http://localhost:8000/api/v1/users \
  -H "Authorization: Bearer $TOKEN"

# Test rate limiting
for i in {1..110}; do
  curl -s http://localhost:8000/api/v1/health
done
```

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use api_gateway::prelude::*;

    #[test]
    fn test_config_loading() {
        let config = Config::load();
        assert!(config.is_ok());
    }
}
```

## Troubleshooting

### Port Already in Use
```bash
# Kill existing process
pkill -f api-gateway

# Or use different port
API_GATEWAY__SERVER__PORT=8001 cargo run -p api-gateway
```

### Backend Service Unavailable
- Check if backend services are running
- Verify service URLs in configuration
- Check logs for connection errors
- Test backend service directly: `curl http://localhost:8001/health`

### Authentication Fails
- Verify JWT secret matches auth service
- Check token expiration timestamp
- Ensure Authorization header format: `Bearer <token>`
- Test token validation: `curl http://localhost:8000/api/v1/auth/verify`

### Rate Limit Issues
- Adjust rate limit in `main.rs`: `RateLimiter::new(limit, seconds)`
- Check client IP detection in logs
- Review rate limit store cleanup

### Build Errors
```bash
# Clean and rebuild
cargo clean
cargo build -p api-gateway

# Update dependencies
cargo update
```

## Performance Considerations

- **Connection Pooling**: HTTP client maintains 10 connections per host
- **Async I/O**: Non-blocking request handling with Tokio
- **Rate Limiting**: In-memory with minimal overhead
- **Timeout**: 30-second request timeout prevents hanging
- **Efficient Routing**: Fast route matching with Actix-web

## Future Enhancements

- [ ] Circuit breaker pattern for service resilience
- [ ] Request caching with Redis
- [ ] API versioning support (v2, v3)
- [ ] WebSocket support for real-time features
- [ ] gRPC gateway for internal services
- [ ] Request/response transformation
- [ ] OpenAPI/Swagger documentation endpoint
- [ ] Advanced rate limiting (per user, per endpoint)
- [ ] Load balancing across multiple instances
- [ ] Request retry logic with exponential backoff
- [ ] Health checks for backend services
- [ ] Metrics endpoint for Prometheus