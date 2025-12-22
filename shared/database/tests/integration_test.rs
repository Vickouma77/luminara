use anyhow::Result;
use shared_database::{create_pool, create_pool_with_retry};
use std::time::Duration;

mod setup;

#[tokio::test]
async fn test_database_connection() -> Result<()> {
    let config = setup::get_test_config();

    // Create pool
    let pool = create_pool(&config).await?;

    // Test connection
    let row: (i32,) = sqlx::query_as("SELECT 1").fetch_one(&pool).await?;

    assert_eq!(row.0, 1);
    Ok(())
}

#[tokio::test]
async fn test_database_connection_with_retry() -> Result<()> {
    let config = setup::get_test_config();

    let pool = create_pool_with_retry(&config, 3, Duration::from_secs(1)).await?;

    let row: (i32,) = sqlx::query_as("SELECT 1").fetch_one(&pool).await?;

    assert_eq!(row.0, 1);
    Ok(())
}

#[tokio::test]
async fn test_health_check() -> Result<()> {
    let config = setup::get_test_config();

    let pool = create_pool(&config).await?;

    let health = shared_database::check_health(&pool).await;
    assert!(health.is_ok());
    Ok(())
}

#[tokio::test]
async fn test_pool_stats() -> Result<()> {
    let config = setup::get_test_config();

    let pool = create_pool(&config).await?;

    let stats = shared_database::pool_stats(&pool);
    // We expect at least min_connections (1)
    assert!(stats.size >= 1);
    Ok(())
}
