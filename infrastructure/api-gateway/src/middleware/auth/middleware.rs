pub(crate) use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    http::header,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::future::{Future, Ready, ready};
use std::pin::Pin;
use std::rc::Rc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // Subject (user ID)
    pub exp: usize,    // Expiration time
    pub iat: usize,    // Issued at
    pub email: String, // User email
}

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            // Extract JWT token from Authorization header
            let token = req
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|h| h.to_str().ok())
                .and_then(|h| h.strip_prefix("Bearer ").map(String::from));

            let Some(token) = token else {
                tracing::warn!("Missing or invalid Authorization header");
                return Err(actix_web::error::ErrorUnauthorized(
                    "Missing authorization token",
                ));
            };

            // Validate JWT token
            let jwt_secret = std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "development-secret-change-in-production".to_string());

            let validation = Validation::new(Algorithm::HS256);
            match decode::<Claims>(
                &token,
                &DecodingKey::from_secret(jwt_secret.as_bytes()),
                &validation,
            ) {
                Ok(token_data) => {
                    tracing::debug!("Authenticated user: {}", token_data.claims.email);

                    // Store claims in request extensions for downstream use
                    req.extensions_mut().insert(token_data.claims);

                    // Continue with the request
                    let res = service.call(req).await?;
                    Ok(res)
                }
                Err(e) => {
                    tracing::warn!("Invalid JWT token: {}", e);
                    Err(actix_web::error::ErrorUnauthorized(
                        "Invalid or expired token",
                    ))
                }
            }
        })
    }
}
