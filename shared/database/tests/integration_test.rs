use shared_database::{create_pool, create_pool_with_retry, DatabaseConfig, Environment, SslMode};
use std::time::Duration;

#[tokio::test]
async fn test_database_connection() {
    // Setup configuration
    // We use the credentials from docker-compose.yml
    // Note: This requires the postgres container to be running
    let url = "postgres://luminara:luminara_dev@localhost:5432/luminara".to_string();
    
    let config = DatabaseConfig {
        url,
        max_connections: 5,
        min_connections: 1,
        connect_timeout: Duration::from_secs(5),
        idle_timeout: Duration::from_secs(60),
        max_lifetime: Duration::from_secs(1800),
        environment: Environment::Development,
        ssl_mode: SslMode::Disable,
    };

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
    let url = "postgres://luminara:luminara_dev@localhost:5432/luminara".to_string();
    
    let config = DatabaseConfig {
        url,
        max_connections: 5,
        min_connections: 1,
        connect_timeout: Duration::from_secs(5),
        idle_timeout: Duration::from_secs(60),
        max_lifetime: Duration::from_secs(1800),
        environment: Environment::Development,
        ssl_mode: SslMode::Disable,
    };

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
    let url = "postgres://luminara:luminara_dev@localhost:5432/luminara".to_string();
    
    let config = DatabaseConfig {
        url,
        max_connections: 5,
        min_connections: 1,
        connect_timeout: Duration::from_secs(5),
        idle_timeout: Duration::from_secs(60),
        max_lifetime: Duration::from_secs(1800),
        environment: Environment::Development,
        ssl_mode: SslMode::Disable,
    };

    let pool = create_pool(&config).await.expect("Failed to create pool");

    let health = shared_database::check_health(&pool).await;
    assert!(health.is_ok());
}

#[tokio::test]
async fn test_pool_stats() {
    let url = "postgres://luminara:luminara_dev@localhost:5432/luminara".to_string();
    
    let config = DatabaseConfig {
        url,
        max_connections: 5,
        min_connections: 1,
        connect_timeout: Duration::from_secs(5),
        idle_timeout: Duration::from_secs(60),
        max_lifetime: Duration::from_secs(1800),
        environment: Environment::Development,
        ssl_mode: SslMode::Disable,
    };

    let pool = create_pool(&config).await.expect("Failed to create pool");

    let stats = shared_database::pool_stats(&pool);
    // We expect at least min_connections (1)
    assert!(stats.size >= 1);
}
