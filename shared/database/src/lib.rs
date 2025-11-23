//! Database utilities and connection pooling

use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

/// Creates a `PostgreSQL` connection pool
///
/// # Errors
///
/// Returns an error if the connection to the database fails or times out
pub async fn create_pool(database_url: &str, max_connections: u32) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(30))
        .connect(database_url)
        .await
}
