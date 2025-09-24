use axum::{
    Router, middleware,
    routing::{delete, get, head},
};

pub mod delete;
pub mod oauth;
pub mod settings;

pub fn register() -> Router {
    let public_routes = Router::new()
        .route("/v1", get(delete::get_user_info))
        .route("/v1/", get(delete::get_user_info))
        .route("/v1/oauth/callback", get(oauth::callback::oauth_callback))
        .route("/v1/oauth/settings", get(oauth::settings::oauth_settings));

    let auth_routes = Router::new()
        .route(
            "/v1/settings",
            head(crate::lib::settings_handlers::handle_head_settings)
                .get(crate::lib::settings_handlers::handle_get_settings)
                .put(crate::lib::settings_handlers::handle_put_settings)
                .delete(crate::lib::settings_handlers::handle_delete_settings),
        )
        .route("/v1", delete(delete::delete_all_user_data))
        .route("/v1/", delete(delete::delete_all_user_data))
        .route_layer(middleware::from_fn(
            crate::middleware::auth::auth_middleware,
        ));

    public_routes.merge(auth_routes)
}
