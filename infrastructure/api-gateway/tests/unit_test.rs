#![allow(clippy::unwrap_used)]

// Unit tests that don't need async runtime
use api_gateway::{Config, RateLimiter};

#[test]
fn test_config_loading() {
    let config = Config::load();
    assert!(config.is_ok(), "Config should load successfully");

    let config = config.unwrap();
    assert_eq!(config.server.host, "0.0.0.0");
    assert_eq!(config.server.port, 8000);
    assert!(!config.services.auth_service_url.is_empty());
    assert!(!config.services.user_service_url.is_empty());
}

#[test]
fn test_rate_limiter_creation() {
    let _limiter = RateLimiter::new(10, 60);
    // Should not panic
}
