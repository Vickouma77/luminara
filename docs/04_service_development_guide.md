# Service Development Guide

## Creating a New Service

This guide shows how to create a new service following the Luminara architecture patterns.

## Step 1: Create Service Structure

```bash
mkdir -p services/my-service/src
cd services/my-service
```

## Step 2: Create Cargo.toml

```toml
[package]
name = "my-service"
edition.workspace = true
version.workspace = true
authors.workspace = true

[dependencies]
# Standard dependencies
tokio.workspace = true
actix-web.workspace = true
actix-cors.workspace = true
serde.workspace = true
serde_json.workspace = true
sqlx.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
tracing-actix-web.workspace = true
uuid.workspace = true
chrono.workspace = true
validator.workspace = true
anyhow.workspace = true
thiserror.workspace = true
config.workspace = true
dotenvy.workspace = true

# Shared libraries
shared-domain.workspace = true
shared-infrastructure.workspace = true
shared-middleware.workspace = true
shared-errors.workspace = true
shared-database.workspace = true
shared-telemetry.workspace = true
shared-events.workspace = true

[lints]
workspace = true
```

## Step 3: Service Structure

```
my-service/
├── Cargo.toml
├── src/
│   ├── main.rs           # Entry point
│   ├── config.rs         # Service configuration
│   ├── api/              # HTTP handlers
│   │   ├── mod.rs
│   │   ├── handlers.rs
│   │   └── routes.rs
│   ├── domain/           # Business logic
│   │   ├── mod.rs
│   │   ├── models.rs
│   │   └── services.rs
│   ├── repository/       # Data access
│   │   ├── mod.rs
│   │   └── postgres.rs
│   └── events/           # Event handlers
│       ├── mod.rs
│       ├── publishers.rs
│       └── subscribers.rs
└── migrations/           # Database migrations
    └── 001_initial.sql
```

## Step 4: Main.rs Template

```rust
use actix_web::{web, App, HttpServer};
use shared_telemetry::init_telemetry;
use shared_middleware::configure_cors;

mod api;
mod config;
mod domain;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize telemetry
    init_telemetry("my-service");
    
    // Load configuration
    let config = config::load_config();
    
    // Setup database pool
    let pool = shared_database::create_pool(
        &config.database_url,
        config.max_connections
    )
    .await
    .expect("Failed to create pool");
    
    tracing::info!("Starting my-service on {}:{}", config.host, config.port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(configure_cors())
            .wrap(tracing_actix_web::TracingLogger::default())
            .configure(api::routes::configure)
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
```

## Step 5: Add to Workspace

Add your service to the root `Cargo.toml`:

```toml
[workspace]
members = [
    # ... existing services
    "services/my-service",
]
```

## Step 6: Database Setup

Create a migration:

```bash
sqlx migrate add initial
```

## Best Practices

1. **Use shared libraries** for common functionality
2. **Follow domain-driven design** principles
3. **Implement proper error handling** using shared-errors
4. **Add comprehensive logging** with tracing
5. **Publish events** for significant domain events
6. **Write tests** for all business logic
7. **Document APIs** with comments
8. **Use validation** for all inputs

## Example API Handler

```rust
use actix_web::{web, HttpResponse};
use shared_errors::AppError;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateRequest {
    pub name: String,
}

pub async fn create(
    pool: web::Data<PgPool>,
    request: web::Json<CreateRequest>,
) -> Result<HttpResponse, AppError> {
    let id = Uuid::new_v4();
    
    // Validate
    if request.name.is_empty() {
        return Err(AppError::BadRequest("Name cannot be empty".into()));
    }
    
    // Save to database
    sqlx::query!(
        "INSERT INTO items (id, name) VALUES ($1, $2)",
        id,
        request.name
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Publish event
    // ...
    
    Ok(HttpResponse::Created().json(id))
}
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_create() {
        // Test implementation
    }
}
```