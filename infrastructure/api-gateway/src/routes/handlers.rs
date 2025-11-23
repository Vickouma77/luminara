use crate::config::Config;
use crate::middleware::auth::AuthMiddleware;
use crate::proxy;
use actix_web::{HttpRequest, HttpResponse, web};
use std::sync::Arc;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Health check endpoint
            .route("/health", web::get().to(health_check))
            // Auth Service routes (public, no auth required for login/register)
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(proxy_to_auth))
                    .route("/register", web::post().to(proxy_to_auth))
                    .route("/refresh", web::post().to(proxy_to_auth))
                    .route("/logout", web::post().to(proxy_to_auth))
                    .route("/verify", web::get().to(proxy_to_auth)),
            )
            // User Service routes (protected)
            .service(
                web::scope("/users")
                    .wrap(AuthMiddleware)
                    .route("", web::get().to(proxy_to_user))
                    .route("", web::post().to(proxy_to_user))
                    .route("/{id}", web::get().to(proxy_to_user))
                    .route("/{id}", web::put().to(proxy_to_user))
                    .route("/{id}", web::delete().to(proxy_to_user))
                    .route("/{id}/profile", web::get().to(proxy_to_user))
                    .route("/{id}/profile", web::put().to(proxy_to_user)),
            )
            // Account Service routes (protected)
            .service(
                web::scope("/accounts")
                    .wrap(AuthMiddleware)
                    .route("", web::get().to(proxy_to_account))
                    .route("", web::post().to(proxy_to_account))
                    .route("/{id}", web::get().to(proxy_to_account))
                    .route("/{id}", web::put().to(proxy_to_account))
                    .route("/{id}", web::delete().to(proxy_to_account))
                    .route("/{id}/balance", web::get().to(proxy_to_account))
                    .route("/{id}/transactions", web::get().to(proxy_to_account)),
            )
            // Transaction Service routes (protected)
            .service(
                web::scope("/transactions")
                    .wrap(AuthMiddleware)
                    .route("", web::get().to(proxy_to_transaction))
                    .route("", web::post().to(proxy_to_transaction))
                    .route("/{id}", web::get().to(proxy_to_transaction))
                    .route("/{id}/status", web::get().to(proxy_to_transaction))
                    .route("/transfer", web::post().to(proxy_to_transaction))
                    .route("/withdraw", web::post().to(proxy_to_transaction))
                    .route("/deposit", web::post().to(proxy_to_transaction)),
            )
            // KYC Service routes (protected)
            .service(
                web::scope("/kyc")
                    .wrap(AuthMiddleware)
                    .route("", web::post().to(proxy_to_kyc))
                    .route("/{id}", web::get().to(proxy_to_kyc))
                    .route("/{id}/documents", web::post().to(proxy_to_kyc))
                    .route("/{id}/status", web::get().to(proxy_to_kyc))
                    .route("/{id}/verify", web::post().to(proxy_to_kyc)),
            )
            // Payment Service routes (protected)
            .service(
                web::scope("/payments")
                    .wrap(AuthMiddleware)
                    .route("", web::get().to(proxy_to_payment))
                    .route("", web::post().to(proxy_to_payment))
                    .route("/{id}", web::get().to(proxy_to_payment))
                    .route("/{id}/status", web::get().to(proxy_to_payment))
                    .route("/{id}/refund", web::post().to(proxy_to_payment))
                    .route("/process", web::post().to(proxy_to_payment)),
            ),
    );
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "api-gateway",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

// Proxy handlers
async fn proxy_to_auth(
    req: HttpRequest,
    body: web::Bytes,
    config: web::Data<Arc<Config>>,
) -> Result<HttpResponse, actix_web::Error> {
    proxy::forward_request(req, body, &config.services.auth_service_url).await
}

async fn proxy_to_user(
    req: HttpRequest,
    body: web::Bytes,
    config: web::Data<Arc<Config>>,
) -> Result<HttpResponse, actix_web::Error> {
    proxy::forward_request(req, body, &config.services.user_service_url).await
}

async fn proxy_to_account(
    req: HttpRequest,
    body: web::Bytes,
    config: web::Data<Arc<Config>>,
) -> Result<HttpResponse, actix_web::Error> {
    proxy::forward_request(req, body, &config.services.account_service_url).await
}

async fn proxy_to_transaction(
    req: HttpRequest,
    body: web::Bytes,
    config: web::Data<Arc<Config>>,
) -> Result<HttpResponse, actix_web::Error> {
    proxy::forward_request(req, body, &config.services.transaction_service_url).await
}

async fn proxy_to_kyc(
    req: HttpRequest,
    body: web::Bytes,
    config: web::Data<Arc<Config>>,
) -> Result<HttpResponse, actix_web::Error> {
    proxy::forward_request(req, body, &config.services.kyc_service_url).await
}

async fn proxy_to_payment(
    req: HttpRequest,
    body: web::Bytes,
    config: web::Data<Arc<Config>>,
) -> Result<HttpResponse, actix_web::Error> {
    proxy::forward_request(req, body, &config.services.payment_service_url).await
}
