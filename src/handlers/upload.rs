use crate::config::app_config::MAX_FILE_SIZE;
use crate::middleware::RateLimiter;
use crate::{AppError, FileMetadata};

use axum::{extract::Multipart, routing::post, Json, Router};
use axum_extra::extract::TypedHeader;
use axum_extra::headers::ContentType;

const ALLOWED_TYPES: [&str; 3] = ["text/plain", "application/json", "application/pdf"];

pub async fn handle_file_upload(
    TypedHeader(content_type): TypedHeader<ContentType>,
    mut multipart: Multipart,
) -> Result<Json<FileMetadata>, AppError> {
    if !content_type.to_string().contains("multipart/form-data") {
        return Err(AppError::ValidationError(
            "Invalid content type, expected multipart/form-data".to_string(),
        ));
    }

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        AppError::ValidationError(format!("Failed to process multipart form: {}", e))
    })? {
        if field.name() != Some("upfile") {
            continue;
        }

        let file_name = field
            .file_name()
            .ok_or_else(|| AppError::ValidationError("No filename provided".to_string()))?
            .to_string();

        let content_type = field
            .content_type()
            .ok_or_else(|| AppError::ValidationError("No content type provided".to_string()))?
            .to_string();

        if !ALLOWED_TYPES.contains(&content_type.as_str()) {
            return Err(AppError::ValidationError(format!(
                "Content type '{}' not allowed",
                content_type
            )));
        }

        let data = field
            .bytes()
            .await
            .map_err(|e| AppError::ValidationError(format!("Failed to read file data: {}", e)))?;

        if data.len() > MAX_FILE_SIZE {
            return Err(AppError::ValidationError(format!(
                "File size exceeds maximum limit of {} bytes",
                MAX_FILE_SIZE
            )));
        }

        return Ok(Json(FileMetadata {
            name: file_name,
            mime_type: content_type,
            size: data.len() as u64,
        }));
    }

    Err(AppError::ValidationError(
        "No file was uploaded".to_string(),
    ))
}

pub fn file_routes() -> Router<RateLimiter> {
    Router::new().route("/upload", post(handle_file_upload))
}
