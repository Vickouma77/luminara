use crate::DatabaseError;
use std::time::Duration;

/// Application environment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Environment {
    #[default]
    Development,
    Staging,
    Production,
}

impl Environment {
    /// Load from `APP_ENV` or `RUST_ENV` environment variable
    #[must_use]
    pub fn from_env() -> Self {
        std::env::var("APP_ENV")
            .or_else(|_| std::env::var("RUST_ENV"))
            .map(|s| s.parse().unwrap_or_default())
            .unwrap_or_default()
    }

    /// Check if running in production
    #[must_use]
    pub fn is_production(&self) -> bool {
        matches!(self, Self::Production)
    }
}

impl std::str::FromStr for Environment {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "production" | "prod" => Self::Production,
            "staging" | "stage" => Self::Staging,
            _ => Self::Development,
        })
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Development => write!(f, "development"),
            Self::Staging => write!(f, "staging"),
            Self::Production => write!(f, "production"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
    pub environment: Environment,
    pub ssl_mode: SslMode,
}

/// SSL mode for database connection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SslMode {
    #[default]
    Prefer,
    Require,
    Disable,
}

impl std::str::FromStr for SslMode {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "require" | "required" => Self::Require,
            "disable" | "disabled" | "off" => Self::Disable,
            _ => Self::Prefer,
        })
    }
}

impl std::fmt::Display for SslMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Prefer => write!(f, "prefer"),
            Self::Require => write!(f, "require"),
            Self::Disable => write!(f, "disable"),
        }
    }
}

impl DatabaseConfig {
    /// Load configuration from environment variables with environment-aware defaults.
    ///
    /// # Environment Variables
    ///
    /// | Variable | Description | Required |
    /// |----------|-------------|----------|
    /// | `DATABASE_URL` | `PostgreSQL` connection string | Yes |
    /// | `APP_ENV` or `RUST_ENV` | Environment (development/staging/production) | No (defaults to development) |
    /// | `DB_MAX_CONNECTIONS` | Maximum pool connections | No (env-based default) |
    /// | `DB_MIN_CONNECTIONS` | Minimum pool connections | No (env-based default) |
    /// | `DB_CONNECT_TIMEOUT_SECS` | Connection timeout in seconds | No (env-based default) |
    /// | `DB_IDLE_TIMEOUT_SECS` | Idle connection timeout in seconds | No (env-based default) |
    /// | `DB_MAX_LIFETIME_SECS` | Max connection lifetime in seconds | No (env-based default) |
    /// | `DB_SSL_MODE` | SSL mode (require/prefer/disable) | No (env-based default) |
    ///
    /// # Errors
    ///
    /// Returns `DatabaseError::ConfigError` if:
    /// - the `DATABASE` environment variable is not set.
    ///
    /// For `DB_MAX_CONNECTIONS` and `DB_MIN_CONNECTION`, defaults are used if missing or invalid.
    pub fn from_env() -> Result<Self, DatabaseError> {
        let environment = Environment::from_env();
        let defaults = EnvironmentDefaults::for_env(environment);

        let url = std::env::var("DATABASE_URL")
            .map_err(|_| DatabaseError::ConfigError("DATABASE_URL not set".into()))?;
        let ssl_mode = std::env::var("DB_SSL_MODE")
            .map(|s| s.parse().unwrap_or(defaults.ssl_mode))
            .unwrap_or(defaults.ssl_mode);
        Ok(Self {
            url,
            max_connections: parse_env_or("DB_MAX_CONNECTIONS", defaults.max_connections),
            min_connections: parse_env_or("DB_MIN_CONNECTIONS", defaults.min_connections),
            connect_timeout: Duration::from_secs(parse_env_or(
                "DB_CONNECT_TIMEOUT_SECS",
                defaults.connect_timeout_secs,
            )),
            idle_timeout: Duration::from_secs(parse_env_or(
                "DB_IDLE_TIMEOUT_SECS",
                defaults.idle_timeout_secs,
            )),
            max_lifetime: Duration::from_secs(parse_env_or(
                "DB_MAX_LIFETIME_SECS",
                defaults.max_lifetime_secs,
            )),
            environment,
            ssl_mode,
        })
    }

    /// Create configuration for a specific environment with explicit URL
    #[must_use]
    pub fn for_environment(url: String, environment: Environment) -> Self {
        let defaults = EnvironmentDefaults::for_env(environment);

        Self {
            url,
            max_connections: defaults.max_connections,
            min_connections: defaults.min_connections,
            connect_timeout: Duration::from_secs(defaults.connect_timeout_secs),
            idle_timeout: Duration::from_secs(defaults.idle_timeout_secs),
            max_lifetime: Duration::from_secs(defaults.max_lifetime_secs),
            environment,
            ssl_mode: defaults.ssl_mode,
        }
    }

    /// Create development configuration with sensible defaults
    #[must_use]
    pub fn development(url: String) -> Self {
        Self::for_environment(url, Environment::Development)
    }

    /// Create a staging configuration
    #[must_use]
    pub fn staging(url: String) -> Self {
        Self::for_environment(url, Environment::Staging)
    }

    /// Create a production configuration with secure defaults
    #[must_use]
    pub fn production(url: String) -> Self {
        Self::for_environment(url, Environment::Production)
    }

    /// Build the full connection URL with SSL mode appended if not already present
    #[must_use]
    pub fn connection_url(&self) -> String {
        if self.url.contains("sslmode=") {
            self.url.clone()
        } else {
            let separator = if self.url.contains('?') { '&' } else { '?' };
            format!("{}{}sslmode={}", self.url, separator, self.ssl_mode)
        }
    }
}

/// Environment-specific default values
struct EnvironmentDefaults {
    max_connections: u32,
    min_connections: u32,
    connect_timeout_secs: u64,
    idle_timeout_secs: u64,
    max_lifetime_secs: u64,
    ssl_mode: SslMode,
}

impl EnvironmentDefaults {
    fn for_env(env: Environment) -> Self {
        match env {
            Environment::Development => Self {
                max_connections: 5,
                min_connections: 1,
                connect_timeout_secs: 30,
                idle_timeout_secs: 300,
                max_lifetime_secs: 1800,
                ssl_mode: SslMode::Disable,
            },
            Environment::Staging => Self {
                max_connections: 10,
                min_connections: 2,
                connect_timeout_secs: 15,
                idle_timeout_secs: 600,
                max_lifetime_secs: 1800,
                ssl_mode: SslMode::Prefer,
            },
            Environment::Production => Self {
                max_connections: 20,
                min_connections: 5,
                connect_timeout_secs: 10,
                idle_timeout_secs: 600,
                max_lifetime_secs: 3600,
                ssl_mode: SslMode::Require,
            },
        }
    }
}

/// Parse an environment or return a default value
fn parse_env_or<T: std::str::FromStr>(key: &str, default: T) -> T {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_environment_parsing() {
        assert_eq!(
            Environment::from_str("production"),
            Ok(Environment::Production)
        );
        assert_eq!(Environment::from_str("prod"), Ok(Environment::Production));
        assert_eq!(Environment::from_str("staging"), Ok(Environment::Staging));
        assert_eq!(Environment::from_str("stage"), Ok(Environment::Staging));
        assert_eq!(
            Environment::from_str("development"),
            Ok(Environment::Development)
        );
        assert_eq!(Environment::from_str("dev"), Ok(Environment::Development));
        assert_eq!(
            Environment::from_str("unknown"),
            Ok(Environment::Development)
        );
    }

    #[test]
    fn test_development_defaults() {
        let config = DatabaseConfig::development("postgres://localhost/test".into());

        assert_eq!(config.max_connections, 5);
        assert_eq!(config.min_connections, 1);
        assert_eq!(config.ssl_mode, SslMode::Disable);
    }

    #[test]
    fn test_production_defaults() {
        let config = DatabaseConfig::production("postgres://db.example.com/app".into());

        assert!(config.connection_url().contains("sslmode=require"));
    }

    #[test]
    fn test_connection_url_preserves_existing_ssl() {
        let config =
            DatabaseConfig::production("postgres://db.example.com/app?sslmode=verify-full".into());

        assert!(!config.connection_url().contains("sslmode=require"));

        assert!(config.connection_url().contains("sslmode=verify-full"));
    }
}
