use crate::{DatabaseConfig, DatabaseError};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

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
