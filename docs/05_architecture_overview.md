# Service Architecture Overview

## Service Map

```
┌─────────────────────────────────────────────────────────────────┐
│                         API Gateway (8000)                       │
│                    Single Entry Point & Routing                  │
└─────────────────────────────────────────────────────────────────┘
                                  │
                ┌─────────────────┼─────────────────┐
                │                 │                 │
                ▼                 ▼                 ▼
    ┌───────────────────┐ ┌──────────────┐ ┌──────────────┐
    │  Auth Service     │ │ User Service │ │Account Service│
    │    Port 8001      │ │  Port 8002   │ │  Port 8003   │
    └───────────────────┘ └──────────────┘ └──────────────┘
                │                 │                 │
    ┌───────────────────┐ ┌──────────────┐ ┌──────────────┐
    │Transaction Service│ │  KYC Service │ │ Notification  │
    │    Port 8004      │ │  Port 8005   │ │Service 8006   │
    └───────────────────┘ └──────────────┘ └──────────────┘
                │                 │                 │
                │         ┌──────────────┐          │
                └────────►│Payment Service│◄────────┘
                          │  Port 8007   │
                          └──────────────┘
                                  │
                ┌─────────────────┼─────────────────┐
                ▼                 ▼                 ▼
        ┌──────────────┐  ┌──────────────┐ ┌──────────────┐
        │  PostgreSQL  │  │    Redis     │ │    Kafka     │
        │   Port 5432  │  │  Port 6379   │ │  Port 9092   │
        └──────────────┘  └──────────────┘ └──────────────┘
```

## Service Dependencies

### Auth Service
- **Database**: auth_service (PostgreSQL)
- **Publishes Events**: 
  - `user.authenticated`
  - `token.refreshed`
  - `password.changed`
- **Dependencies**: None (foundational service)

### User Service
- **Database**: user_service (PostgreSQL)
- **Publishes Events**: 
  - `user.created`
  - `user.updated`
  - `user.deleted`
- **Subscribes To**: 
  - `user.authenticated` (update last login)
- **Dependencies**: Auth Service (for authentication)

### Account Service
- **Database**: account_service (PostgreSQL)
- **Publishes Events**: 
  - `account.created`
  - `account.updated`
  - `account.closed`
- **Subscribes To**: 
  - `user.created` (create default accounts)
  - `transaction.completed` (update balance)
- **Dependencies**: User Service, Auth Service

### Transaction Service
- **Database**: transaction_service (PostgreSQL)
- **Publishes Events**: 
  - `transaction.initiated`
  - `transaction.completed`
  - `transaction.failed`
- **Subscribes To**: 
  - `account.created` (setup transaction history)
  - `payment.received` (record external payments)
- **Dependencies**: Account Service, Auth Service

### KYC Service
- **Database**: kyc_service (PostgreSQL)
- **Publishes Events**: 
  - `kyc.submitted`
  - `kyc.approved`
  - `kyc.rejected`
- **Subscribes To**: 
  - `user.created` (initiate KYC process)
- **Dependencies**: User Service, Auth Service
- **External APIs**: ID verification services

### Notification Service
- **Database**: notification_service (PostgreSQL)
- **Publishes Events**: 
  - `notification.sent`
  - `notification.failed`
- **Subscribes To**: 
  - All major events (wildcard subscriber)
- **Dependencies**: Auth Service
- **External APIs**: Email, SMS, Push notification providers

### Payment Service
- **Database**: payment_service (PostgreSQL)
- **Publishes Events**: 
  - `payment.initiated`
  - `payment.completed`
  - `payment.failed`
  - `payment.refunded`
- **Subscribes To**: 
  - `transaction.completed` (process external payments)
  - `account.created` (setup payment methods)
- **Dependencies**: Account Service, Transaction Service, Auth Service
- **External APIs**: Payment gateways (Stripe, PayPal, etc.)

## Shared Libraries

### shared-domain
**Purpose**: Domain models and business rules  
**Used By**: All services  
**Provides**:
- Entity traits
- Common entities (User, Account, etc.)
- Value objects (Money, Email, etc.)
- Domain enums (Status types, etc.)

### shared-infrastructure
**Purpose**: Infrastructure utilities  
**Used By**: All services  
**Provides**:
- Configuration management
- HTTP client factory
- Common utilities

### shared-middleware
**Purpose**: HTTP middleware components  
**Used By**: All services with HTTP APIs  
**Provides**:
- Authentication middleware
- CORS configuration
- Request logging

### shared-errors
**Purpose**: Error handling  
**Used By**: All services  
**Provides**:
- AppError enum
- HTTP error responses
- Error conversions

### shared-events
**Purpose**: Event definitions  
**Used By**: All services using events  
**Provides**:
- Event trait
- Event DTOs
- Event metadata

### shared-database
**Purpose**: Database utilities  
**Used By**: All services with databases  
**Provides**:
- Connection pooling
- Common queries
- Database utilities

### shared-telemetry
**Purpose**: Observability  
**Used By**: All services  
**Provides**:
- Logging setup
- Metrics collection
- Distributed tracing

## API Gateway

**Purpose**: Single entry point for all client requests  
**Port**: 8000  
**Responsibilities**:
- Request routing
- Load balancing
- Rate limiting
- Authentication verification
- Request/response transformation
- API versioning

**Routes**:
- `/api/v1/auth/*` → Auth Service
- `/api/v1/users/*` → User Service
- `/api/v1/accounts/*` → Account Service
- `/api/v1/transactions/*` → Transaction Service
- `/api/v1/kyc/*` → KYC Service
- `/api/v1/payments/*` → Payment Service

## Event Bus

**Purpose**: Asynchronous communication between services  
**Technology**: Kafka (optional), or custom event system  
**Responsibilities**:
- Event publishing
- Event subscription
- Event routing
- Guaranteed delivery
- Event replay

## Data Flow Examples

### Example 1: User Registration
1. Client → API Gateway → User Service: Create user
2. User Service → Database: Store user
3. User Service → Event Bus: Publish `user.created`
4. KYC Service ← Event Bus: Subscribe to `user.created`
5. KYC Service: Initiate KYC process
6. Account Service ← Event Bus: Subscribe to `user.created`
7. Account Service: Create default account
8. Account Service → Event Bus: Publish `account.created`
9. Notification Service ← Event Bus: Subscribe to `user.created`
10. Notification Service: Send welcome email

### Example 2: Money Transfer
1. Client → API Gateway → Transaction Service: Initiate transfer
2. Transaction Service → Account Service: Verify balances
3. Transaction Service → Database: Create transaction
4. Transaction Service → Event Bus: Publish `transaction.initiated`
5. Transaction Service: Process transaction
6. Transaction Service → Account Service: Update balances
7. Transaction Service → Event Bus: Publish `transaction.completed`
8. Notification Service ← Event Bus: Subscribe to `transaction.completed`
9. Notification Service: Send confirmation notifications

## Deployment Strategy

### Local Development
- Docker Compose for infrastructure
- Cargo for building and running services
- Each service runs independently

### Production
- Kubernetes for orchestration
- Each service in its own pod
- Horizontal scaling based on load
- Service mesh for communication (optional)

## Security Considerations

1. **Authentication**: JWT tokens issued by Auth Service
2. **Authorization**: Role-based access control in each service
3. **API Gateway**: First line of defense
4. **Service-to-Service**: Mutual TLS (optional)
5. **Database**: Encrypted connections
6. **Secrets**: Environment variables or secret manager
7. **Rate Limiting**: Per-user and per-IP limits

## Performance Considerations

1. **Caching**: Redis for frequently accessed data
2. **Database**: Connection pooling, read replicas
3. **Async**: Non-blocking I/O throughout
4. **Event Processing**: Batch processing where possible
5. **Load Balancing**: Multiple instances per service
6. **CDN**: Static assets and API gateway

## Monitoring & Observability

1. **Logs**: Structured JSON logs, centralized in ELK/Loki
2. **Metrics**: Prometheus for time-series data
3. **Traces**: Jaeger for distributed tracing
4. **Dashboards**: Grafana for visualization
5. **Alerts**: Prometheus Alertmanager
6. **Health Checks**: `/health` and `/ready` endpoints

## Future Enhancements

1. gRPC for inter-service communication
2. GraphQL API layer
3. CQRS pattern for read/write separation
4. Event sourcing for audit trail
5. Service mesh (Istio/Linkerd)
6. Advanced circuit breakers
7. A/B testing framework
8. Feature flags system