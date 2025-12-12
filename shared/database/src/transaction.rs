use crate::DatabaseError;
use sqlx::{PgPool, Postgres, Transaction};
use std::future::Future;

/// Execute operations within a database transaction
///
/// # Errors
///
/// Returns `DatabaseError` if the transaction fails to begin, commit, or if the closure returns an error.
pub async fn with_transaction<'a, F, Fut, T>(pool: &'a PgPool, f: F) -> Result<T, DatabaseError>
where
    F: FnOnce(Transaction<'a, Postgres>) -> Fut,
    Fut: Future<Output = Result<(Transaction<'a, Postgres>, T), DatabaseError>>,
{
    let tx = pool.begin().await?;
    match f(tx).await {
        Ok((tx, result)) => {
            tx.commit().await?;
            Ok(result)
        }
        Err(e) => Err(e),
    }
}
