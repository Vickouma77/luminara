use actix_web::{HttpRequest, HttpResponse};
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
    // Convert actix-web Method to reqwest Method via string
    let method = reqwest::Method::from_bytes(req.method().as_str().as_bytes())
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid HTTP method"))?;
    let mut forward_req = HTTP_CLIENT.request(method, &url);

    // Forward headers (except Host and Connection)
    for (header_name, header_value) in req.headers() {
        let name = header_name.as_str();
        if name != "host" && name != "connection" {
            // Convert actix-web header value to reqwest header value via bytes
            if let Ok(value) = reqwest::header::HeaderValue::from_bytes(header_value.as_bytes()) {
                forward_req = forward_req.header(name, value);
            }
        }
    }

    // Send the request with body
    let response = forward_req.body(body.to_vec()).send().await.map_err(|e| {
        tracing::error!("Failed to forward request: {}", e);
        actix_web::error::ErrorBadGateway(format!("Service unavailable: {e}"))
    })?;

    // Build response
    // Convert reqwest StatusCode to actix-web StatusCode via u16
    let status_code = actix_web::http::StatusCode::from_u16(response.status().as_u16())
        .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
    let mut client_resp = HttpResponse::build(status_code);

    // Copy headers from service response
    for (header_name, header_value) in response.headers() {
        let name = header_name.as_str();
        if name != "connection" {
            // Convert reqwest header value to actix-web header value via bytes
            if let Ok(value) =
                actix_web::http::header::HeaderValue::from_bytes(header_value.as_bytes())
            {
                client_resp.insert_header((name, value));
            }
        }
    }

    // Get response body
    let body = response.bytes().await.map_err(|e| {
        tracing::error!("Failed to read response body: {}", e);
        actix_web::error::ErrorBadGateway("Failed to read service response")
    })?;

    Ok(client_resp.body(body))
}
