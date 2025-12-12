use crate::DatabaseError;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait Repository<T>: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<T>, DatabaseError>;
    async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<T>, DatabaseError>;
    async fn create(&self, entity: T) -> Result<T, DatabaseError>;
    async fn update(&self, entity: T) -> Result<T, DatabaseError>;
    async fn delete(&self, id: Uuid) -> Result<bool, DatabaseError>;
}

#[derive(Clone)]
pub struct BaseRepository {
    pool: PgPool,
}

impl BaseRepository {
    #[must_use]
    pub fn pg_pool(&self) -> &PgPool {
        &self.pool
    }
}
