use crate::{DatabaseConfig, DatabaseError, create_pool};
use sqlx::PgPool;

/// Database pools for primary and optional read replica.
///
/// Use this when you need to scale read operations by directing
/// them to a read replica while writes go to the primary.
#[derive(Clone)]
pub struct DatabasePool {
    primary: PgPool,
    replica: Option<PgPool>,
}

impl DatabasePool {
    /// Create pools with only a primary database.
    #[must_use]
    pub fn primary_only(primary: PgPool) -> Self {
        Self {
            primary,
            replica: None,
        }
    }

    /// Create pools with primary and read replica.
    #[must_use]
    pub fn with_replica(primary: PgPool, replica: PgPool) -> Self {
        Self {
            primary,
            replica: Some(replica),
        }
    }

    /// Create pools from configuration.
    ///
    /// If `replica_config` is provided, a replica pool will be created.
    ///
    /// # Errors
    ///
    /// Returns `DatabaseError::ConnectionError` if pool creation fails.
    pub async fn from_config(
        primary_config: &DatabaseConfig,
        replica_config: Option<&DatabaseConfig>,
    ) -> Result<Self, DatabaseError> {
        let primary = create_pool(primary_config).await?;

        let replica = match replica_config {
            Some(config) => {
                tracing::info!("Creating read replica pool...");
                Some(create_pool(config).await?)
            }
            None => None,
        };
        Ok(Self { primary, replica })
    }

    /// Get the primary pool for write operations.
    #[must_use]
    pub fn primary(&self) -> &PgPool {
        &self.primary
    }

    /// Get the read pool (replica if available, otherwise primary).
    ///
    /// Use this for read-only queries that can be served by a replica.
    #[must_use]
    pub fn read(&self) -> &PgPool {
        self.replica.as_ref().unwrap_or(&self.primary)
    }

    /// Get the replica pool if available.
    #[must_use]
    pub fn replica(&self) -> Option<&PgPool> {
        self.replica.as_ref()
    }

    /// Check if a replica is configured.
    #[must_use]
    pub fn has_replica(&self) -> bool {
        self.replica.is_some()
    }

    /// Close all pools gracefully.
    pub async fn close(self) {
        tracing::info!("Closing database pools...");

        self.primary.close().await;

        if let Some(replica) = self.replica {
            replica.close().await;
        }

        tracing::info!("All database pools closed");
    }
}
