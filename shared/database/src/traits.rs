use crate::DatabaseError;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

/// Generic repository trait for CRUD operations.
///
/// Implement this trait for your domain entities to get
/// standardized database access patterns.
#[async_trait]
pub trait Repository<T>: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<T>, DatabaseError>;
    async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<T>, DatabaseError>;
    async fn count(&self) -> Result<u64, DatabaseError>;
    async fn create(&self, entity: T) -> Result<T, DatabaseError>;
    async fn update(&self, entity: T) -> Result<T, DatabaseError>;
    async fn delete(&self, id: Uuid) -> Result<bool, DatabaseError>;
}

/// Soft delete support for entities that should not be permanently removed.
#[async_trait]
pub trait SoftDeletable<T>: Repository<T> {
    async fn soft_delete(&self, id: Uuid) -> Result<bool, DatabaseError>;
    async fn restore(&self, id: Uuid) -> Result<bool, DatabaseError>;
    async fn find_all_including_deleted(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<T>, DatabaseError>;
}

/// Base repository providing common functionality for all repositories.
#[derive(Clone)]
pub struct BaseRepository {
    pool: PgPool,
}

impl BaseRepository {
    /// Create a new `BaseRepository` with the given connection pool.
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get a reference to the underlying connection pool.
    #[must_use]
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Consume the repository and return the underlying pool.
    #[must_use]
    pub fn into_pool(self) -> PgPool {
        self.pool
    }
}
