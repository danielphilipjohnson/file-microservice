// tests/file_upload.rs
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    routing::post,
    Router,
};
use file_microservice::{
    handlers::upload::handle_file_upload, init_rate_limiter, models::FileMetadata,
};
use tower::ServiceExt;

#[tokio::test]
async fn test_successful_file_upload() {
    println!("Starting file upload test");
    let boundary = "test_boundary";
    let body = format!(
        "--{boundary}\r\n\
         Content-Disposition: form-data; name=\"upfile\"; filename=\"test.txt\"\r\n\
         Content-Type: text/plain\r\n\r\n\
         test content\r\n\
         --{boundary}--\r\n",
    );

    let req = Request::builder()
        .uri("/upload")
        .method("POST")
        .header(
            "content-type",
            format!("multipart/form-data; boundary={boundary}"),
        )
        .body(Body::from(body))
        .unwrap();

    // Create test app
    let app = Router::new()
        .route("/upload", post(handle_file_upload))
        .with_state(init_rate_limiter());

    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Set a reasonable size limit (e.g., 5MB)
    let body = to_bytes(response.into_body(), 5 * 1024 * 1024)
        .await
        .unwrap();
    let metadata: FileMetadata = serde_json::from_slice(&body).unwrap();

    assert_eq!(metadata.name, "test.txt");
    assert_eq!(metadata.mime_type, "text/plain");
    assert_eq!(metadata.size, 12);
}

#[tokio::test]
async fn test_file_too_large() {
    let large_content = "x".repeat(6 * 1024 * 1024); // 6MB
    let boundary = "test_boundary";
    let body = format!(
        "--{boundary}\r\n\
         Content-Disposition: form-data; name=\"upfile\"; filename=\"large.txt\"\r\n\
         Content-Type: text/plain\r\n\r\n\
         {large_content}\r\n\
         --{boundary}--\r\n",
    );

    let req = Request::builder()
        .uri("/upload")
        .method("POST")
        .header(
            "content-type",
            format!("multipart/form-data; boundary={boundary}"),
        )
        .body(Body::from(body))
        .unwrap();

    let app = Router::new()
        .route("/upload", post(handle_file_upload))
        .with_state(init_rate_limiter());

    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_missing_file() {
    let boundary = "test_boundary";
    let body = format!(
        "--{boundary}\r\n\
         Content-Disposition: form-data; name=\"wrong_field\"\r\n\
         Content-Type: text/plain\r\n\r\n\
         some content\r\n\
         --{boundary}--\r\n",
    );

    let req = Request::builder()
        .uri("/upload")
        .method("POST")
        .header(
            "content-type",
            format!("multipart/form-data; boundary={boundary}"),
        )
        .body(Body::from(body))
        .unwrap();

    let app = Router::new()
        .route("/upload", post(handle_file_upload))
        .with_state(init_rate_limiter());

    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_invalid_content_type() {
    println!("Starting invalid content type test"); // Add this
    let boundary = "test_boundary";
    let body = format!(
        "--{boundary}\r\n\
         Content-Disposition: form-data; name=\"upfile\"; filename=\"test.exe\"\r\n\
         Content-Type: application/x-msdownload\r\n\r\n\
         some content\r\n\
         --{boundary}--\r\n",
    );

    let req = Request::builder()
        .uri("/upload")
        .method("POST")
        .header(
            "content-type",
            format!("multipart/form-data; boundary={boundary}"),
        )
        .body(Body::from(body))
        .unwrap();

    let app = Router::new()
        .route("/upload", post(handle_file_upload))
        .with_state(init_rate_limiter());

    let response = app.oneshot(req).await.unwrap();
    println!("Response status: {:?}", response.status()); // Add this

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Add response body logging
    let body = to_bytes(response.into_body(), 5 * 1024 * 1024)
        .await
        .unwrap();
    println!("Response body: {:?}", String::from_utf8_lossy(&body));
}
