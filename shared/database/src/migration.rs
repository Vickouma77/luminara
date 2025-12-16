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

    let applied_count = get_applied_migration_count(pool).await?;
    let total_migrations = migrator.migrations.len();

    Ok(total_migrations > applied_count)
}

/// Get information about applied migrations.
///
/// # Errors
///
/// Returns `DatabaseError::ConnectionError` if the query fails.
pub async fn get_applied_migrations(
    pool: &PgPool,
) -> Result<Vec<AppliedMigrations>, DatabaseError> {
    let result: Result<Vec<(i64, String)>, _> =
        sqlx::query_as("SELECT version, description FROM _sqlx_migrations ORDER BY version")
            .fetch_all(pool)
            .await;

    match result {
        Ok(rows) => Ok(rows
            .into_iter()
            .map(|(version, description)| AppliedMigrations {
                version,
                description,
            })
            .collect()),
        Err(e) if is_table_not_found(&e) => Ok(Vec::new()),
        Err(e) => Err(DatabaseError::ConnectionError(e)),
    }
}

/// Get the count of applied migrations.
///
/// Returns 0 if no migrations have been applied yet (table doesn't exist).
///
/// # Errors
///
/// Returns `DatabaseError::ConnectionError` if the query fails.
async fn get_applied_migration_count(pool: &PgPool) -> Result<usize, DatabaseError> {
    let result: Result<(i64,), _> =
        sqlx::query_as("SELECT COUNT(*) FROM _sqlx_migrations")
            .fetch_one(pool)
            .await;

    match result {
        Ok((count,)) => Ok(count as usize),
        Err(e) if is_table_not_found(&e) => Ok(0),
        Err(e) => Err(DatabaseError::ConnectionError(e)),
    }
}

/// Check if the error indicates the migrations table doesn't exist.
///
/// This is expected on first run before any migrations have been applied.
fn is_table_not_found(error: &sqlx::Error) -> bool {
    match error {
        sqlx::Error::Database(db_err) => {
            // PostgreSQL error code 42P01 = undefined_table
            db_err.code().map_or(false, |code| code == "42P01")
        }
        _ => false,
    }
}

/// Information about an applied migration.
#[derive(Debug, Clone)]
pub struct AppliedMigrations {
    /// Migration version number
    pub version: i64,
    /// Migration description
    pub description: String,
}
