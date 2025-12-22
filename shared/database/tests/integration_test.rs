use shared_database::{DatabaseConfig, Environment, SslMode, create_pool, create_pool_with_retry};
use std::time::Duration;

fn get_test_config() -> DatabaseConfig {
    let url = "postgres://luminara:luminara_dev@localhost:5432/luminara".to_string();

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

#[tokio::test]
async fn test_database_connection() {
    let config = get_test_config();

    // Create pool
    let pool = create_pool(&config).await.expect("Failed to create pool");

    // Test connection
    let row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(&pool)
        .await
        .expect("Failed to execute query");

    assert_eq!(row.0, 1);
}

#[tokio::test]
async fn test_database_connection_with_retry() {
    let config = get_test_config();

    let pool = create_pool_with_retry(&config, 3, Duration::from_secs(1))
        .await
        .expect("Failed to create pool with retry");

    let row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(&pool)
        .await
        .expect("Failed to execute query");

    assert_eq!(row.0, 1);
}

#[tokio::test]
async fn test_health_check() {
    let config = get_test_config();

    let pool = create_pool(&config).await.expect("Failed to create pool");

    let health = shared_database::check_health(&pool).await;
    assert!(health.is_ok());
}

#[tokio::test]
async fn test_pool_stats() {
    let config = get_test_config();

    let pool = create_pool(&config).await.expect("Failed to create pool");

    let stats = shared_database::pool_stats(&pool);
    // We expect at least min_connections (1)
    assert!(stats.size >= 1);
}
