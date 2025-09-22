use axum::Router;

pub mod health;

pub fn register_routes() -> Router {
    Router::new().merge(health::register())
}
