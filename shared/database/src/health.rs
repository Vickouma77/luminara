use crate::DatabaseError;
use sqlx::PgPool;

/// Check database connectivity kubernetes probes
///
/// # Errors
///
/// Returns `DatabaseError::ConnectionError` if the database connection fails.
pub async fn check_health(pool: &PgPool) -> Result<(), DatabaseError> {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await
        .map_err(DatabaseError::ConnectionError)?;

    Ok(())
}

/// Get pool statistics from monitoring
#[must_use]
pub fn pool_stats(pool: &PgPool) -> PoolStats {
    PoolStats {
        size: pool.size(),
        idle: pool.num_idle(),
    }
}

#[derive(Debug)]
pub struct PoolStats {
    pub size: u32,
    pub idle: usize,
}
