use crate::{
    api::appstate::AppState, api::auth::google_oauth_handler,
    api::healthcheck::health_checker_handler,
};
use axum::{routing::get, Router};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/healthcheck", get(health_checker_handler))
        .route("/oauth2/google", get(google_oauth_handler))
        .with_state(state)
}
