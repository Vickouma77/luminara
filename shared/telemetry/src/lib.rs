//! Telemetry and Observability

use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_telemetry(service_name: &str) {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{service_name}=debug,tower_http=debug").into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();
}

pub fn init_metrics() {
    // Placeholder for Prometheus metrics setup
}
