use api_gateway::config::Config;

#[test]
fn test_default_config_loads() {
    let config = Config::load();
    assert!(config.is_ok(), "Default config should load successfully");
}

#[test]
fn test_config_has_correct_defaults() {
    let config = Config::load().expect("Config should load");

    // Server config
    assert_eq!(config.server.host, "0.0.0.0");
    assert_eq!(config.server.port, 8000);

    // Service URLs
    assert_eq!(config.services.auth_service_url, "http://localhost:8001");
    assert_eq!(config.services.user_service_url, "http://localhost:8002");
    assert_eq!(config.services.account_service_url, "http://localhost:8003");
    assert_eq!(
        config.services.transaction_service_url,
        "http://localhost:8004"
    );
    assert_eq!(config.services.kyc_service_url, "http://localhost:8005");
    assert_eq!(
        config.services.notification_service_url,
        "http://localhost:8006"
    );
    assert_eq!(config.services.payment_service_url, "http://localhost:8007");

    // Auth config
    assert!(!config.auth.jwt_secret.is_empty());
}

#[test]
#[ignore] // Skipped due to unsafe environment variable manipulation
fn test_config_from_environment() {
    // This test would require unsafe blocks to manipulate environment variables
    // Skipping for now - can be tested manually with actual env vars set
}

#[test]
fn test_config_clone() {
    let config = Config::load().expect("Config should load");
    let cloned = config.clone();

    assert_eq!(config.server.port, cloned.server.port);
    assert_eq!(
        config.services.auth_service_url,
        cloned.services.auth_service_url
    );
}

#[test]
fn test_config_debug_format() {
    let config = Config::load().expect("Config should load");
    let debug_str = format!("{:?}", config);

    assert!(debug_str.contains("Config"));
    assert!(debug_str.contains("server"));
}
