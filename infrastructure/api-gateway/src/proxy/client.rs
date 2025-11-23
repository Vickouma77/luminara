use actix_web::{HttpRequest, HttpResponse, http::header};
use reqwest::Client;
use std::sync::LazyLock;
use std::time::Duration;

static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .build()
        .unwrap_or_else(|_| Client::new())
});

/// Forward an HTTP request to a backend service
///
/// # Errors
///
/// Returns an error if the request fails or the service is unavailable
pub async fn forward_request(
    req: HttpRequest,
    body: actix_web::web::Bytes,
    target_url: &str,
) -> Result<HttpResponse, actix_web::Error> {
    // Extract path from original request
    let path = req.uri().path();
    let query = req.uri().query().unwrap_or("");

    // Build target URL
    let url = if query.is_empty() {
        format!("{target_url}{path}")
    } else {
        format!("{target_url}{path}?{query}")
    };

    tracing::debug!("Forwarding {} request to: {}", req.method(), url);

    // Build the forwarded request
    let mut forward_req = HTTP_CLIENT.request(req.method().clone(), &url);

    // Forward headers (except Host and Connection)
    for (header_name, header_value) in req.headers() {
        let name = header_name.as_str();
        if name != "host" && name != "connection" {
            forward_req = forward_req.header(name, header_value);
        }
    }

    // Send the request with body
    let response = forward_req.body(body.to_vec()).send().await.map_err(|e| {
        tracing::error!("Failed to forward request: {}", e);
        actix_web::error::ErrorBadGateway(format!("Service unavailable: {e}"))
    })?;

    // Build response
    let status = response.status();
    let mut client_resp = HttpResponse::build(status);

    // Copy headers from service response
    for (header_name, header_value) in response.headers() {
        if header_name != header::CONNECTION {
            client_resp.insert_header((header_name.clone(), header_value.clone()));
        }
    }

    // Get response body
    let body = response.bytes().await.map_err(|e| {
        tracing::error!("Failed to read response body: {}", e);
        actix_web::error::ErrorBadGateway("Failed to read service response")
    })?;

    Ok(client_resp.body(body))
}
