use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Connection failed: {0}")]
    ConnectionError(#[from] sqlx::Error),

    #[error("Record not found: {0}")]
    NotFound(String),

    #[error("Duplicate entry: {0}")]
    DuplicateEntry(String),

    #[error("Transaction Failed: {0}")]
    TransactionError(String),

    #[error("Migration Failed: {0}")]
    MigrationError(String),

    #[error("Configuration Error: {0}")]
    ConfigError(String),
}
