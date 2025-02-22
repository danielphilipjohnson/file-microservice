// tests/api_health.rs
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    Router,
};
use file_microservice::{
    handlers::health::{health_check, metrics_handler},
    init_rate_limiter,
};
use serde_json::Value;
use tower::ServiceExt;

const MAX_RESPONSE_SIZE: usize = 1024 * 1024; // 1MB limit for responses

#[tokio::test]
async fn test_health_check() {
    // Initialize the router with the health route
    let app = Router::new()
        .route("/health", axum::routing::get(health_check))
        .with_state(init_rate_limiter());

    // Create a test request
    let request = Request::builder()
        .uri("/health")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    // Send the request and get response
    let response = app.oneshot(request).await.unwrap();

    // Check status code
    assert_eq!(response.status(), StatusCode::OK);

    // Get response body with size limit
    let body = to_bytes(response.into_body(), MAX_RESPONSE_SIZE)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    // Verify response structure
    assert!(json.get("status").is_some());
    assert_eq!(json["status"], "healthy");
    assert!(json.get("timestamp").is_some());
    assert!(json["timestamp"].as_u64().is_some());
}

#[tokio::test]
async fn test_metrics_endpoint() {
    // Initialize the router with the metrics route
    let app = Router::new()
        .route("/metrics", axum::routing::get(metrics_handler))
        .with_state(init_rate_limiter());

    // Create a test request
    let request = Request::builder()
        .uri("/metrics")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    // Send the request and get response
    let response = app.oneshot(request).await.unwrap();

    // Check status code
    assert_eq!(response.status(), StatusCode::OK);

    // Get response body with size limit
    let body = to_bytes(response.into_body(), MAX_RESPONSE_SIZE)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    // Verify metrics structure
    assert!(json.get("uptime_seconds").is_some());
    assert!(json.get("memory_usage_mb").is_some());
    assert!(json.get("cpu_usage_percent").is_some());
    assert!(json.get("total_requests").is_some());
    assert!(json.get("requests_per_minute").is_some());

    // Verify value ranges
    assert!(json["memory_usage_mb"].as_u64().unwrap() >= 50); // Base memory is 50
    assert!(json["cpu_usage_percent"].as_f64().unwrap() >= 0.0);
    assert!(json["cpu_usage_percent"].as_f64().unwrap() <= 100.0);
}

#[tokio::test]
async fn test_multiple_health_requests() {
    for _ in 0..3 {
        // Create a new router for each request
        let app = Router::new()
            .route("/health", axum::routing::get(health_check))
            .with_state(init_rate_limiter());

        let request = Request::builder()
            .uri("/health")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Verify response body for each request with size limit
        let body = to_bytes(response.into_body(), MAX_RESPONSE_SIZE)
            .await
            .unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["status"], "healthy");
    }
}

#[tokio::test]
async fn test_metrics_changes_over_time() {
    let app = Router::new()
        .route("/metrics", axum::routing::get(metrics_handler))
        .with_state(init_rate_limiter());

    // First request
    let initial_metrics = get_metrics(&app).await;

    // Wait a second
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Second request
    let updated_metrics = get_metrics(&app).await;

    // Verify metrics changed
    assert!(
        updated_metrics["uptime_seconds"].as_u64().unwrap()
            > initial_metrics["uptime_seconds"].as_u64().unwrap()
    );
}

#[tokio::test]
async fn test_invalid_endpoints() {
    let base_app = Router::new()
        .route("/health", axum::routing::get(health_check))
        .route("/metrics", axum::routing::get(metrics_handler))
        .with_state(init_rate_limiter());

    // Test non-existent endpoint
    let request = Request::builder()
        .uri("/nonexistent")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = base_app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // Test wrong method
    let request = Request::builder()
        .uri("/health")
        .method("POST")
        .body(Body::empty())
        .unwrap();

    let response = base_app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}

// Helper function to get metrics
async fn get_metrics(app: &Router) -> Value {
    let request = Request::builder()
        .uri("/metrics")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), MAX_RESPONSE_SIZE)
        .await
        .unwrap();
    serde_json::from_slice(&body).unwrap()
}
