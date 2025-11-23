use config::{Config as ConfigBuilder, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub services: ServicesConfig,
    #[allow(dead_code)]
    pub auth: AuthConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServicesConfig {
    pub auth_service_url: String,
    pub user_service_url: String,
    pub account_service_url: String,
    pub transaction_service_url: String,
    pub kyc_service_url: String,
    pub payment_service_url: String,
    #[allow(dead_code)]
    pub notification_service_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    #[allow(dead_code)]
    pub jwt_secret: String,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let run_env = std::env::var("RUN_ENV").unwrap_or_else(|_| "development".into());

        ConfigBuilder::builder()
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8000)?
            .set_default("services.auth_service_url", "http://localhost:8001")?
            .set_default("services.user_service_url", "http://localhost:8002")?
            .set_default("services.account_service_url", "http://localhost:8003")?
            .set_default("services.transaction_service_url", "http://localhost:8004")?
            .set_default("services.kyc_service_url", "http://localhost:8005")?
            .set_default("services.payment_service_url", "http://localhost:8007")?
            .set_default("services.notification_service_url", "http://localhost:8006")?
            .set_default("auth.jwt_secret", "development-secret-change-in-production")?
            .add_source(File::with_name(&format!("config/{}", run_env)).required(false))
            .add_source(File::with_name("config/default").required(false))
            .add_source(Environment::with_prefix("API_GATEWAY").separator("__"))
            .build()?
            .try_deserialize()
    }
}
