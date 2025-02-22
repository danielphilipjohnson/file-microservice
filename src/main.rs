mod config;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;

pub use error::AppError;
use file_microservice::config::app_config::MAX_FILE_SIZE;
pub use middleware::{init_rate_limiter, rate_limit_middleware, RateLimiter};
pub use models::FileMetadata;

use axum::{middleware::from_fn_with_state, routing::get, Router};

use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::handlers::metrics_handler;
use crate::handlers::{file_routes, health_routes, init_metrics};

#[tokio::main]
async fn main() {
    let rate_limiter = init_rate_limiter();

    init_metrics();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .merge(file_routes())
        .merge(health_routes())
        .route("/metrics", get(metrics_handler))
        .layer(TraceLayer::new_for_http())
        .layer(RequestBodyLimitLayer::new(MAX_FILE_SIZE))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(from_fn_with_state(
            rate_limiter.clone(),
            rate_limit_middleware,
        ))
        .with_state(rate_limiter);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
