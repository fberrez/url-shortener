use axum::{extract::State, response::Redirect};
use axum_macros::debug_handler;

use crate::api::appstate::AppState;

#[debug_handler]
pub async fn google_oauth_handler(state: State<AppState>) -> Redirect {
    let redirect_url = state.config.oauth2_google_redirect_url.as_str();
    let client_id = state.config.oauth2_google_client_id.as_str();
    let url = format!("https://accounts.google.com/o/oauth2/v2/auth?client_id={client_id}&redirect_uri={redirect_url}&response_type=code&scope=https%3A//www.googleapis.com/auth/userinfo.email");
    println!("Redirecting to: {}", url);
    Redirect::permanent(&url)
}

pub async fn google_oauth_callback_handler() {
    // TODO
}
