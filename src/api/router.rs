use crate::api::{
    appstate::AppState,
    auth::{google_oauth_callback_handler, google_oauth_handler},
    healthcheck::health_checker_handler,
};
use axum::{routing::get, Extension, Router};

#[derive(Clone, Debug)]
pub struct UserData {
    pub user_id: i64,
    pub user_email: String,
}

pub fn create_router(state: AppState) -> Router {
    let user_data: Option<UserData> = None;

    Router::new()
        .route("/api/healthcheck", get(health_checker_handler))
        .route("/oauth2/google", get(google_oauth_handler))
        .route(
            "/oauth2/google/callback",
            get(google_oauth_callback_handler),
        )
        .with_state(state)
        .layer(Extension(user_data))
}
