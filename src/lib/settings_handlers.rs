use axum::{
    http::HeaderMap,
    response::IntoResponse,
    Extension,
    body::Bytes,
};

use crate::lib::DatabaseService;
use crate::routes::v1::settings::{head_settings, get_settings, put_settings, delete_settings};

pub async fn handle_head_settings(
    Extension(db): Extension<DatabaseService>,
    Extension(user_id): Extension<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    head_settings(Extension(db), headers, user_id).await
}

pub async fn handle_get_settings(
    Extension(db): Extension<DatabaseService>,
    Extension(user_id): Extension<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    get_settings(Extension(db), headers, user_id).await
}

pub async fn handle_put_settings(
    Extension(db): Extension<DatabaseService>,
    Extension(user_id): Extension<String>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    put_settings(Extension(db), headers, user_id, body.to_vec()).await
}

pub async fn handle_delete_settings(
    Extension(db): Extension<DatabaseService>,
    Extension(user_id): Extension<String>,
) -> impl IntoResponse {
    delete_settings(Extension(db), user_id).await
}