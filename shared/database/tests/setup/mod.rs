use shared_database::{DatabaseConfig, Environment, SslMode};
use std::time::Duration;

pub fn get_test_config() -> DatabaseConfig {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://luminara:luminara_dev@localhost:5432/luminara".to_string());

    DatabaseConfig {
        url,
        max_connections: 5,
        min_connections: 1,
        connect_timeout: Duration::from_secs(5),
        idle_timeout: Duration::from_secs(60),
        max_lifetime: Duration::from_secs(1800),
        environment: Environment::Development,
        ssl_mode: SslMode::Disable,
    }
}
