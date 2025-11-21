//! API Gateway Library
//! 
//! This library provides the core functionality for the Luminara Banking Platform API Gateway.
//! It includes routing, authentication, rate limiting, and request proxying capabilities.

pub mod config;
pub mod middleware;
pub mod proxy;
pub mod routes;

// Re-export commonly used types for convenience
pub use config::Config;
pub use middleware::auth::AuthMiddleware;
pub use middleware::rate_limit::RateLimiter;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::config::Config;
    pub use crate::middleware::auth::AuthMiddleware;
    pub use crate::middleware::rate_limit::RateLimiter;
    pub use crate::routes;
    pub use crate::proxy;
}
