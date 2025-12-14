use crate::DatabaseError;
use sqlx::PgPool;
use sqlx::migrate::Migrator;
use std::path::Path;

/// Run database migrations from the specified directory.
///
/// # Arguments
///
/// * `pool` - Database connection pool
/// * `migrations_path` - Path to the migrations directory
///
/// # Errors
///
/// Returns `DatabaseError::MigrationError` if migrations fail to run.
///
/// # Example
///
/// ```ignore
/// use shared_database::{create_pool, run_migrations, DatabaseConfig};
///
/// let config = DatabaseConfig::from_env()?;
/// let pool = create_pool(&config).await?;
/// run_migrations(&pool, "./migrations").await?;
/// ```
pub async fn run_migrations<P: AsRef<Path>>(
    pool: &PgPool,
    migrations_path: P,
) -> Result<(), DatabaseError> {
    tracing::info!("Running migrations for {:?}...", migrations_path.as_ref());

    Migrator::new(migrations_path.as_ref())
        .await
        .map_err(|e| DatabaseError::MigrationError(e.to_string()))?
        .run(pool)
        .await
        .map_err(|e| DatabaseError::MigrationError(e.to_string()))?;

    tracing::info!("Database completed successfully!.");

    Ok(())
}

/// Check if there are pending migrations.
///
/// # Errors
///
/// Returns `DatabaseError::MigrationError` if unable to check migration status.
pub async fn has_pending_migrations<P: AsRef<Path>>(
    pool: &PgPool,
    migrations_path: P,
) -> Result<bool, DatabaseError> {
    let migrator = Migrator::new(migrations_path.as_ref())
        .await
        .map_err(|e| DatabaseError::MigrationError(e.to_string()))?;

    let applied: Vec<(i64,)> =
        sqlx::query_as("SELECT version FROM _sqlx_migrations ORDER BY version")
            .fetch_all(pool)
            .await
            .unwrap_or_default();

    let total_migrations = migrator.migrations.len();

    Ok(total_migrations > applied.len())
}

/// Get information about applied migrations.
///
/// # Errors
///
/// Returns `DatabaseError::ConnectionError` if the query fails.
pub async fn get_applied_migrations(
    pool: &PgPool,
) -> Result<Vec<AppliedMigrations>, DatabaseError> {
    let rows: Vec<(i64, String)> =
        sqlx::query_as("SELECT version, description FROM _sqlx_migrations ORDER BY version")
            .fetch_all(pool)
            .await
            .unwrap_or_default();

    Ok(rows
        .into_iter()
        .map(|(version, description)| AppliedMigrations {
            version,
            description,
        })
        .collect())
}

/// Information about an applied migration.
#[derive(Debug, Clone)]
pub struct AppliedMigrations {
    /// Migration version number
    pub version: i64,
    /// Migration description
    pub description: String,
}
