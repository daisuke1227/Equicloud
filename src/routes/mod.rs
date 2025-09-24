use axum::Router;

pub mod health;
pub mod v1;

pub fn register_routes() -> Router {
    Router::new()
        .merge(health::register())
        .merge(v1::register())
}
