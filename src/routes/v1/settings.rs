use axum::{
    body::Body,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Extension,
};
use serde_json::json;
use std::env;
use tracing::error;

use crate::lib::DatabaseService;

pub async fn head_settings(
    Extension(db): Extension<DatabaseService>,
    _headers: HeaderMap,
    user_id: String,
) -> impl IntoResponse {
    match db.get_settings_metadata(&user_id).await {
        Ok(Some(written)) => {
            let mut response_headers = HeaderMap::new();
            if let Ok(etag_value) = written.parse() {
                response_headers.insert("ETag", etag_value);
            } else {
                error!("Failed to parse ETag value: {}", written);
            }
            (StatusCode::NO_CONTENT, response_headers)
        }
        Ok(None) => (StatusCode::NOT_FOUND, HeaderMap::new()),
        Err(e) => {
            error!("Database error in head_settings: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, HeaderMap::new())
        }
    }
}

pub async fn get_settings(
    Extension(db): Extension<DatabaseService>,
    headers: HeaderMap,
    user_id: String,
) -> impl IntoResponse {
    match db.get_user_settings(&user_id).await {
        Ok(Some((value, written))) => {
            if let Some(if_none_match) = headers.get("if-none-match") {
                if if_none_match.to_str().unwrap_or("") == written {
                    return (StatusCode::NOT_MODIFIED, HeaderMap::new(), Body::empty())
                        .into_response();
                }
            }

            let mut response_headers = HeaderMap::new();
            if let Ok(content_type) = "application/octet-stream".parse() {
                response_headers.insert("Content-Type", content_type);
            }
            if let Ok(etag_value) = written.parse() {
                response_headers.insert("ETag", etag_value);
            } else {
                error!("Failed to parse ETag value: {}", written);
            }

            (StatusCode::OK, response_headers, Body::from(value)).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, HeaderMap::new(), Body::empty()).into_response(),
        Err(e) => {
            error!("Database error in get_settings: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                HeaderMap::new(),
                Body::empty(),
            )
                .into_response()
        }
    }
}

pub async fn put_settings(
    Extension(db): Extension<DatabaseService>,
    headers: HeaderMap,
    user_id: String,
    body: Vec<u8>,
) -> impl IntoResponse {
    if headers.get("content-type").and_then(|h| h.to_str().ok()) != Some("application/octet-stream")
    {
        return (
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            axum::Json(json!({
                "error": "Content type must be application/octet-stream"
            })),
        )
            .into_response();
    }

    let size_limit = env::var("MAX_BACKUP_SIZE_BYTES")
        .unwrap_or_else(|_| "62914560".to_string())
        .parse::<usize>()
        .unwrap_or(62914560);

    if body.len() > size_limit {
        return (
            StatusCode::PAYLOAD_TOO_LARGE,
            axum::Json(json!({
                "error": "Settings are too large"
            })),
        )
            .into_response();
    }

    match db.save_user_settings(&user_id, body).await {
        Ok(written) => (
            StatusCode::OK,
            axum::Json(json!({
                "written": written
            })),
        )
            .into_response(),
        Err(e) => {
            error!("Database error in put_settings: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(json!({
                    "error": "Failed to save settings"
                })),
            )
                .into_response()
        }
    }
}

pub async fn delete_settings(
    Extension(db): Extension<DatabaseService>,
    user_id: String,
) -> impl IntoResponse {
    match db.delete_user_settings(&user_id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) => {
            error!("Database error in delete_settings: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

