use axum::response::IntoResponse;
use axum::{
    body::Body,
    extract::{ConnectInfo, State},
    http::{HeaderValue, Request},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use std::time::{Duration, Instant};

use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::Mutex;

use crate::AppError;

#[derive(Clone)]
pub struct RateLimitConfig {
    pub requests: u32,
    pub window: Duration,
}

#[derive(Clone)]
struct TokenBucket {
    tokens: u32,
    last_refill: Instant,
}

#[derive(Clone)]
pub struct RateLimiter {
    buckets: Arc<Mutex<HashMap<String, TokenBucket>>>,
    config: RateLimitConfig,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            buckets: Arc::new(Mutex::new(HashMap::new())),
            config,
        }
    }

    async fn get_rate_limit_info(&self, key: &str) -> (bool, u32, Duration) {
        let mut buckets = self.buckets.lock().await;
        let now = Instant::now();

        let bucket = buckets.entry(key.to_string()).or_insert(TokenBucket {
            tokens: self.config.requests,
            last_refill: now,
        });

        // Calculate tokens to refill based on elapsed time
        let elapsed = now.duration_since(bucket.last_refill);
        let tokens_to_add = (elapsed.as_secs_f32() / self.config.window.as_secs_f32()
            * self.config.requests as f32) as u32;

        if tokens_to_add > 0 {
            bucket.tokens = (bucket.tokens + tokens_to_add).min(self.config.requests);
            bucket.last_refill = now;
        }

        let is_limited = bucket.tokens == 0;
        let remaining_tokens = if is_limited { 0 } else { bucket.tokens - 1 };

        // Calculate time until window reset
        let time_passed = now.duration_since(bucket.last_refill);
        let time_until_reset = self.config.window.saturating_sub(time_passed);

        if !is_limited {
            bucket.tokens = remaining_tokens;
        }

        (is_limited, remaining_tokens, time_until_reset)
    }

    pub async fn cleanup(&self) {
        let mut buckets = self.buckets.lock().await;
        let now = Instant::now();
        buckets.retain(|_, bucket| now.duration_since(bucket.last_refill) < self.config.window * 2);
    }
}

pub async fn rate_limit_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(rate_limiter): State<RateLimiter>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let key = addr.ip().to_string();

    // Get rate limit information
    let (is_limited, remaining_calls, time_until_reset) =
        rate_limiter.get_rate_limit_info(&key).await;

    if is_limited {
        let mut response = AppError::RateLimitExceeded.into_response();

        let headers = response.headers_mut();
        headers.insert(
            "X-RateLimit-Limit",
            HeaderValue::from(rate_limiter.config.requests),
        );
        headers.insert("X-RateLimit-Remaining", HeaderValue::from(0));
        headers.insert(
            "X-RateLimit-Reset",
            HeaderValue::from(time_until_reset.as_secs()),
        );

        return response;
    }

    // Process the request
    let mut response = next.run(request).await;

    let headers = response.headers_mut();
    headers.insert(
        "X-RateLimit-Limit",
        HeaderValue::from(rate_limiter.config.requests),
    );
    headers.insert("X-RateLimit-Remaining", HeaderValue::from(remaining_calls));
    headers.insert(
        "X-RateLimit-Reset",
        HeaderValue::from(time_until_reset.as_secs()),
    );

    response
}

pub fn init_rate_limiter() -> RateLimiter {
    let config = RateLimitConfig {
        requests: 5,
        window: Duration::from_secs(60),
    };

    let rate_limiter = RateLimiter::new(config);
    let rate_limiter_clone = rate_limiter.clone();

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            rate_limiter_clone.cleanup().await;
        }
    });

    rate_limiter
}
