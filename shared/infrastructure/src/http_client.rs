//! HTTP client utilities

/// Creates a new HTTP client
///
/// # Must Use
///
/// The returned client should be reused across requests for connection pooling
#[must_use]
pub fn create_http_client() -> reqwest::Client {
    reqwest::Client::new()
}
