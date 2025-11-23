#![allow(clippy::expect_used)]

use actix_web::{App, HttpServer, middleware::Logger, web};
use api_gateway::prelude::*;
use shared_telemetry::init_telemetry;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize telemetry and logging
    init_telemetry("api-gateway");

    // Load configuration
    let config = Config::load().expect("Failed to load configuration");
    let bind_address = format!("{}:{}", config.server.host, config.server.port);

    tracing::info!("Starting API Gateway on {}", bind_address);
    tracing::info!("Service endpoints configured:");
    tracing::info!("  - Auth Service: {}", config.services.auth_service_url);
    tracing::info!("  - User Service: {}", config.services.user_service_url);
    tracing::info!(
        "  - Account Service: {}",
        config.services.account_service_url
    );
    tracing::info!(
        "  - Transaction Service: {}",
        config.services.transaction_service_url
    );
    tracing::info!("  - KYC Service: {}", config.services.kyc_service_url);
    tracing::info!(
        "  - Payment Service: {}",
        config.services.payment_service_url
    );

    let config = Arc::new(config);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .wrap(Logger::default())
            .wrap(shared_middleware::configure_cors())
            .wrap(RateLimiter::new(100, 60)) // 100 requests per 60 seconds
            .configure(routes::configure)
    })
    .bind(&bind_address)?
    .run()
    .await
}
