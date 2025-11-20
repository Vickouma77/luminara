//! Telemetry and Observability

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init_telemetry(service_name: &str) {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!("{}=debug,tower_http=debug", service_name).into()
        }))
        .with(tracing_subscriber::fmt::layer().json())
        .init();
}

pub fn init_metrics() {
    // Placeholder for Prometheus metrics setup
}
