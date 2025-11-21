use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use std::collections::HashMap;
use std::future::{ready, Ready, Future};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::rc::Rc;

#[derive(Clone)]
struct RateLimitEntry {
    count: usize,
    reset_time: Instant,
}

#[derive(Clone)]
pub struct RateLimiter {
    max_requests: usize,
    window_secs: u64,
    store: Arc<Mutex<HashMap<String, RateLimitEntry>>>,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        Self {
            max_requests,
            window_secs,
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn check_rate_limit(&self, client_id: &str) -> Result<(), ()> {
        let mut store = self.store.lock().unwrap();
        let now = Instant::now();

        let entry = store.entry(client_id.to_string()).or_insert_with(|| {
            RateLimitEntry {
                count: 0,
                reset_time: now + Duration::from_secs(self.window_secs),
            }
        });

        // Reset if window has passed
        if now >= entry.reset_time {
            entry.count = 0;
            entry.reset_time = now + Duration::from_secs(self.window_secs);
        }

        // Check limit
        if entry.count >= self.max_requests {
            return Err(());
        }

        entry.count += 1;
        Ok(())
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimiterMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterMiddleware {
            service: Rc::new(service),
            limiter: self.clone(),
        }))
    }
}

pub struct RateLimiterMiddleware<S> {
    service: Rc<S>,
    limiter: RateLimiter,
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
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
        let limiter = self.limiter.clone();

        Box::pin(async move {
            // Use IP address as client identifier
            let client_id = req
                .connection_info()
                .realip_remote_addr()
                .unwrap_or("unknown")
                .to_string();

            match limiter.check_rate_limit(&client_id) {
                Ok(_) => {
                    let res = service.call(req).await?;
                    Ok(res)
                }
                Err(_) => {
                    tracing::warn!("Rate limit exceeded for client: {}", client_id);
                    Err(actix_web::error::ErrorTooManyRequests(
                        "Rate limit exceeded. Please try again later."
                    ))
                }
            }
        })
    }
}
