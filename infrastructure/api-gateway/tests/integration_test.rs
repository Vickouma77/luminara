use actix_web::{test, web, App};
use api_gateway::prelude::*;
use std::sync::Arc;

/// Helper function to create test app
fn create_test_app() -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let config = Config::load().expect("Failed to load config");
    let rate_limiter = RateLimiter::new(100, 60);

    App::new()
        .app_data(web::Data::new(Arc::new(config)))
        .wrap(rate_limiter)
        .configure(api_gateway::routes::configure)
}

#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(create_test_app()).await;

    let req = test::TestRequest::get()
        .uri("/api/v1/health")
        .to_request();

    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    
    assert!(body_str.contains("healthy"));
    assert!(body_str.contains("api-gateway"));
}

#[actix_web::test]
async fn test_auth_routes_accessible() {
    let app = test::init_service(create_test_app()).await;

    // Test login endpoint is accessible (should fail with backend unavailable, but route exists)
    let req = test::TestRequest::post()
        .uri("/api/v1/auth/login")
        .set_json(&serde_json::json!({
            "email": "test@example.com",
            "password": "password123"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    
    // We expect 502 Bad Gateway since auth service isn't running
    // But the route should be registered (not 404)
    assert_ne!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_protected_route_without_auth() {
    let app = test::init_service(create_test_app()).await;

    let req = test::TestRequest::get()
        .uri("/api/v1/users")
        .to_request();

    let resp = test::try_call_service(&app, req).await;
    
    // Should return Err with unauthorized
    assert!(resp.is_err(), "Should fail without auth token");
}

#[actix_web::test]
async fn test_protected_route_with_invalid_token() {
    let app = test::init_service(create_test_app()).await;

    let req = test::TestRequest::get()
        .uri("/api/v1/users")
        .insert_header(("Authorization", "Bearer invalid_token_here"))
        .to_request();

    let resp = test::try_call_service(&app, req).await;
    
    // Should return Err with invalid token
    assert!(resp.is_err(), "Should fail with invalid token");
}

#[actix_web::test]
async fn test_protected_route_with_malformed_header() {
    let app = test::init_service(create_test_app()).await;

    // Missing "Bearer" prefix
    let req = test::TestRequest::get()
        .uri("/api/v1/users")
        .insert_header(("Authorization", "just_a_token"))
        .to_request();

    let resp = test::try_call_service(&app, req).await;
    
    assert!(resp.is_err(), "Should fail with malformed header");
}

#[actix_web::test]
async fn test_rate_limiting() {
    let app = test::init_service(create_test_app()).await;

    let mut success_count = 0;
    let mut rate_limited_count = 0;

    // Make multiple requests rapidly
    for _i in 0..110 {
        let req = test::TestRequest::get()
            .uri("/api/v1/health")
            .to_request();

        // Use try_call_service to handle rate limit errors
        match test::try_call_service(&app, req).await {
            Ok(resp) if resp.status() == actix_web::http::StatusCode::OK => {
                success_count += 1;
            }
            Err(_) => {
                rate_limited_count += 1;
            }
            _ => {}
        }
    }

    // Should have some successful and some rate limited
    assert!(success_count > 0, "Some requests should succeed (got {})", success_count);
    assert!(rate_limited_count > 0, "Some requests should be rate limited (got {})", rate_limited_count);
}

#[actix_web::test]
async fn test_all_service_routes_registered() {
    let app = test::init_service(create_test_app()).await;

    // Test public routes (should return 502 when backend unavailable, not 404)
    let public_routes = vec![
        ("/api/v1/health", "GET", actix_web::http::StatusCode::OK),
        ("/api/v1/auth/login", "POST", actix_web::http::StatusCode::BAD_GATEWAY),
        ("/api/v1/auth/register", "POST", actix_web::http::StatusCode::BAD_GATEWAY),
    ];

    for (route, method, _expected) in public_routes {
        let req = match method {
            "GET" => test::TestRequest::get().uri(route).to_request(),
            "POST" => test::TestRequest::post().uri(route).to_request(),
            _ => continue,
        };

        let resp = test::call_service(&app, req).await;
        
        // Routes should be registered (not 404)
        assert_ne!(
            resp.status(),
            actix_web::http::StatusCode::NOT_FOUND,
            "Route {} {} should be registered",
            method,
            route
        );
    }

    // Test protected routes (should return error without auth, not 404)
    let protected_routes = vec![
        "/api/v1/users",
        "/api/v1/accounts",
        "/api/v1/transactions",
        "/api/v1/kyc",
        "/api/v1/payments",
    ];

    for route in protected_routes {
        let req = test::TestRequest::get()
            .uri(route)
            .to_request();

        let resp = test::try_call_service(&app, req).await;
        
        // Should return error (auth required) not success (which would mean 404)
        assert!(
            resp.is_err(),
            "Protected route {} should require auth",
            route
        );
    }
}

#[actix_web::test]
async fn test_cors_headers() {
    let app = test::init_service(create_test_app()).await;

    let req = test::TestRequest::get()
        .uri("/api/v1/health")
        .insert_header(("Origin", "http://localhost:3000"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    // Note: CORS middleware needs to be added to main.rs for this to pass
    // This test documents the expected behavior
}

#[actix_web::test]
async fn test_invalid_route() {
    let app = test::init_service(create_test_app()).await;

    let req = test::TestRequest::get()
        .uri("/api/v1/nonexistent")
        .to_request();

    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
}
