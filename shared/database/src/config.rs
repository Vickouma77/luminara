use crate::DatabaseError;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
}

impl DatabaseConfig {
    /// Load configuration from environment variables.
    ///
    /// # Errors
    ///
    /// Returns `DatabaseError::ConfigError` if:
    /// - the `DATABASE` environment variable is not set.
    ///
    /// For `DB_MAX_CONNECTIONS` and `DB_MIN_CONNECTION`, defaults are used if missing or invalid.
    pub fn from_env() -> Result<Self, DatabaseError> {
        Ok(Self {
            url: std::env::var("DATABASE")
                .map_err(|_| DatabaseError::ConfigError("DATABASE_URL not set".into()))?,
            max_connections: std::env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".into())
                .parse()
                .unwrap_or(10),
            min_connections: std::env::var("DB_MIN_CONNECTION")
                .unwrap_or_else(|_| "2".into())
                .parse()
                .unwrap_or(2),
            connect_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(30),
            max_lifetime: Duration::from_secs(1800),
        })
    }
}
