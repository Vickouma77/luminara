use crate::{DatabaseConfig, DatabaseError};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

/// Create a new `PostgreSQL` connection pool.
///
/// # Errors
///
/// Returns `DatabaseError::ConnectionError` if the connection to the database fails.
pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool, DatabaseError> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(config.connect_timeout)
        .idle_timeout(config.idle_timeout)
        .max_lifetime(config.max_lifetime)
        .connect(&config.url)
        .await?;

    tracing::info!(
        "Database pool created: max={}, min={}",
        config.max_connections,
        config.min_connections
    );

    Ok(pool)
}

/// Create a new `PostgreSQL` connection pool with retry logic.
///
/// This is useful for microservices that may start before the database is ready,
/// such as during container orchestration or rolling deployments.
///
/// # Arguments
///
/// * `config` - Database configuration
/// * `max_retries` - Maximum number of connection attempts
/// * `retry_delay` - Duration to wait between retry attempts
///
/// # Errors
///
/// Returns `DatabaseError::ConnectionError` if all retry attempts fail.
///
/// # Example
///
/// ```ignore
/// use shared_database::{DatabaseConfig, create_pool_with_retry};
/// use std::time::Duration;
///
/// let config = DatabaseConfig::from_env()?;
/// let pool = create_pool_with_retry(&config, 5, Duration::from_secs(2)).await?;
/// ```
pub async fn create_pool_with_retry(
    config: &DatabaseConfig,
    max_retries: u32,
    retry_delay: Duration,
) -> Result<PgPool, DatabaseError> {
    let mut last_error = None;

    for attempt in 1..=max_retries {
        tracing::info!(
            "Attempting database connection (attempt {}/{})",
            attempt,
            max_retries
        );

        match create_pool(config).await {
            Ok(pool) => {
                tracing::info!("Database connection established on attempt {}", attempt);
                return Ok(pool);
            }
            Err(e) => {
                tracing::warn!("Database connection attempt {} failed: {}", attempt, e);
                last_error = Some(e);

                if attempt < max_retries {
                    tracing::info!("Retrying in {:?}...", retry_delay);
                    tokio::time::sleep(retry_delay).await;
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        DatabaseError::ConnectionError(sqlx::Error::Configuration("Max retries exceeded".into()))
    }))
}

/// Gracefully close the connection pool.
///
/// This should be called during application shutdown to ensure
/// all connections are properly closed.
pub async fn close_pool(pool: PgPool) {
    tracing::info!("Closing database connection pool...");
    pool.close().await;
    tracing::info!("Database connection pool closed");
}
