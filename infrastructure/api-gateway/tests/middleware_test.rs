#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::cast_possible_truncation,
    clippy::uninlined_format_args
)]

use actix_web::{App, HttpResponse, test, web};
use api_gateway::{AuthMiddleware, RateLimiter};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
    email: String,
}

/// Helper to generate a valid JWT token for testing
fn generate_test_token() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = Claims {
        sub: "test-user-id".to_string(),
        exp: now + 3600, // 1 hour from now
        iat: now,
        email: "test@example.com".to_string(),
    };

    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "development-secret-change-in-production".to_string());

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("Failed to generate token")
}

/// Helper to generate an expired JWT token
fn generate_expired_token() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = Claims {
        sub: "test-user-id".to_string(),
        exp: now - 3600, // Expired 1 hour ago
        iat: now - 7200,
        email: "test@example.com".to_string(),
    };

    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "development-secret-change-in-production".to_string());

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("Failed to generate token")
}

async fn protected_handler() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Protected resource accessed"
    }))
}

#[actix_web::test]
async fn test_auth_middleware_with_valid_token() {
    let app = test::init_service(
        App::new().service(
            web::scope("/api")
                .wrap(AuthMiddleware)
                .route("/protected", web::get().to(protected_handler)),
        ),
    )
    .await;

    let token = generate_test_token();

    let req = test::TestRequest::get()
        .uri("/api/protected")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
}

#[actix_web::test]
async fn test_auth_middleware_without_token() {
    let app = test::init_service(
        App::new().service(
            web::scope("/api")
                .wrap(AuthMiddleware)
                .route("/protected", web::get().to(protected_handler)),
        ),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/protected").to_request();

    let resp = test::try_call_service(&app, req).await;

    assert!(resp.is_err(), "Should fail without token");
}

#[actix_web::test]
async fn test_auth_middleware_with_expired_token() {
    let app = test::init_service(
        App::new().service(
            web::scope("/api")
                .wrap(AuthMiddleware)
                .route("/protected", web::get().to(protected_handler)),
        ),
    )
    .await;

    let token = generate_expired_token();

    let req = test::TestRequest::get()
        .uri("/api/protected")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::try_call_service(&app, req).await;

    assert!(resp.is_err(), "Should fail with expired token");
}

#[actix_web::test]
async fn test_auth_middleware_with_malformed_token() {
    let app = test::init_service(
        App::new().service(
            web::scope("/api")
                .wrap(AuthMiddleware)
                .route("/protected", web::get().to(protected_handler)),
        ),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/protected")
        .insert_header(("Authorization", "Bearer invalid.token.here"))
        .to_request();

    let resp = test::try_call_service(&app, req).await;

    assert!(resp.is_err(), "Should fail with malformed token");
}

#[actix_web::test]
async fn test_auth_middleware_without_bearer_prefix() {
    let app = test::init_service(
        App::new().service(
            web::scope("/api")
                .wrap(AuthMiddleware)
                .route("/protected", web::get().to(protected_handler)),
        ),
    )
    .await;

    let token = generate_test_token();

    let req = test::TestRequest::get()
        .uri("/api/protected")
        .insert_header(("Authorization", token)) // Missing "Bearer " prefix
        .to_request();

    let resp = test::try_call_service(&app, req).await;

    assert!(resp.is_err(), "Should fail without Bearer prefix");
}

async fn public_handler() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Public resource"
    }))
}

#[actix_web::test]
async fn test_rate_limiter_allows_requests_within_limit() {
    let limiter = RateLimiter::new(5, 60);

    let app = test::init_service(
        App::new()
            .wrap(limiter)
            .route("/public", web::get().to(public_handler)),
    )
    .await;

    // Make 5 requests (within limit)
    for i in 0..5 {
        let req = test::TestRequest::get().uri("/public").to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(
            resp.status(),
            actix_web::http::StatusCode::OK,
            "Request {} should succeed",
            i
        );
    }
}

#[actix_web::test]
async fn test_rate_limiter_blocks_excess_requests() {
    let limiter = RateLimiter::new(3, 60);

    let app = test::init_service(
        App::new()
            .wrap(limiter)
            .route("/public", web::get().to(public_handler)),
    )
    .await;

    let mut success_count = 0;
    let mut failed_count = 0;

    // Make 5 requests (exceeding limit of 3)
    for _i in 0..5 {
        let req = test::TestRequest::get().uri("/public").to_request();

        match test::try_call_service(&app, req).await {
            Ok(_) => success_count += 1,
            Err(_) => failed_count += 1,
        }
    }

    assert_eq!(success_count, 3, "First 3 requests should succeed");
    assert_eq!(failed_count, 2, "Last 2 requests should be rate limited");
}

#[actix_web::test]
async fn test_combined_middleware_auth_and_rate_limit() {
    let limiter = RateLimiter::new(10, 60);

    let app = test::init_service(
        App::new().wrap(limiter).service(
            web::scope("/api")
                .wrap(AuthMiddleware)
                .route("/protected", web::get().to(protected_handler)),
        ),
    )
    .await;

    let token = generate_test_token();

    // Valid token should pass both middleware
    let req = test::TestRequest::get()
        .uri("/api/protected")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    // Invalid token should fail at auth middleware (before rate limit)
    let req = test::TestRequest::get()
        .uri("/api/protected")
        .insert_header(("Authorization", "Bearer invalid"))
        .to_request();

    let resp = test::try_call_service(&app, req).await;

    assert!(resp.is_err(), "Should fail with invalid token");
}
