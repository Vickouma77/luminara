use shared_database::{create_pool, create_pool_with_retry};
use std::time::Duration;

mod setup;

#[tokio::test]
async fn test_database_connection() {
    let config = setup::get_test_config();

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
    let config = setup::get_test_config();

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
    let config = setup::get_test_config();

    let pool = create_pool(&config).await.expect("Failed to create pool");

    let health = shared_database::check_health(&pool).await;
    assert!(health.is_ok());
}

#[tokio::test]
async fn test_pool_stats() {
    let config = setup::get_test_config();

    let pool = create_pool(&config).await.expect("Failed to create pool");

    let stats = shared_database::pool_stats(&pool);
    // We expect at least min_connections (1)
    assert!(stats.size >= 1);
}
