// inspiration: https://github.com/randommm/rust-axum-with-google-oauth/blob/master/src/routes/oauth.rs

use std::collections::HashMap;

use axum::{
    extract::{Host, Query, State},
    response::{IntoResponse, Redirect},
    Extension,
};
use chrono::Utc;
use oauth2::{
    basic::BasicClient, reqwest::http_client, AuthUrl, AuthorizationCode, ClientId, ClientSecret,
    CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RevocationUrl, Scope,
    TokenResponse, TokenUrl,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::api::appstate::AppState;

use super::{error_handling::AppError, router::UserData};

fn get_client(state: AppState, hostname: String) -> Result<BasicClient, AppError> {
    let google_client_id = ClientId::new(state.config.oauth2_google_client_id);
    let google_client_secret = ClientSecret::new(state.config.oauth2_google_client_secret);
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .map_err(|_| "OAuth: invalid authorization endpoint URL")?;
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .map_err(|_| "OAuth: invalid token endpoint URL")?;

    // Set up the config for the Google OAuth2 process.
    let client = BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(
        RedirectUrl::new(state.config.oauth2_google_redirect_url)
            .map_err(|_| "OAuth: invalid redirect URL")?,
    )
    .set_revocation_uri(
        RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
            .map_err(|_| "OAuth: invalid revocation endpoint URL")?,
    );
    Ok(client)
}

pub async fn google_oauth_handler(
    Extension(user_data): Extension<Option<UserData>>,
    State(state): State<AppState>,
    Host(hostname): Host,
) -> Result<Redirect, AppError> {
    if user_data.is_some() {
        return Ok(Redirect::to("/"));
    }

    let client = get_client(state.clone(), hostname)?;

    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    let return_url = "/";
    sqlx::query(
        "INSERT INTO oauth2_state_storage (csrf_state, pkce_code_verifier, return_url) VALUES ($1, $2, $3);",
    )
    .bind(csrf_state.secret())
    .bind(pkce_code_verifier.secret())
    .bind(return_url)
    .execute(&state.pool)
    .await?;

    Ok(Redirect::to(authorize_url.as_str()))
}

pub async fn google_oauth_callback_handler(
    Query(mut params): Query<HashMap<String, String>>,
    State(app_state): State<AppState>,
    Host(hostname): Host,
) -> Result<impl IntoResponse, AppError> {
    let state = CsrfToken::new(params.remove("state").ok_or("OAuth: without state")?);
    let code = AuthorizationCode::new(params.remove("code").ok_or("OAuth: without code")?);

    let query: (String, String) = sqlx::query_as(
    r#"DELETE FROM oauth2_state_storage WHERE csrf_state = $1 RETURNING pkce_code_verifier,return_url"#,
    )
    .bind(state.secret())
    .fetch_one(&app_state.pool)
    .await?;

    let pkce_code_verifier = query.0;
    let return_url = query.1;
    let pkce_code_verifier = PkceCodeVerifier::new(pkce_code_verifier);

    // Exchange the code with a token.
    let client = get_client(app_state.clone(), hostname)?;
    let token_response = tokio::task::spawn_blocking(move || {
        client
            .exchange_code(code)
            .set_pkce_verifier(pkce_code_verifier)
            .request(http_client)
    })
    .await
    .map_err(|_| "OAuth: exchange_code failure")?
    .map_err(|_| "OAuth: tokio spawn blocking failure")?;
    let access_token = token_response.access_token().secret();

    // Get user info from Google
    let url =
        "https://www.googleapis.com/oauth2/v2/userinfo?oauth_token=".to_owned() + access_token;
    let body = reqwest::get(url)
        .await
        .map_err(|_| "OAuth: reqwest failed to query userinfo")?
        .text()
        .await
        .map_err(|_| "OAuth: reqwest received invalid userinfo")?;
    let mut body: serde_json::Value =
        serde_json::from_str(body.as_str()).map_err(|_| "OAuth: Serde failed to parse userinfo")?;
    let email = body["email"]
        .take()
        .as_str()
        .ok_or("OAuth: Serde failed to parse email address")?
        .to_owned();
    let verified_email = body["verified_email"]
        .take()
        .as_bool()
        .ok_or("OAuth: Serde failed to parse verified_email")?;
    if !verified_email {
        return Err(AppError::new("OAuth: email address is not verified".to_owned())
            .with_user_message("Your email address is not verified. Please verify your email address with Google and try again.".to_owned()));
    }

    // Check if user exists in database
    // If not, create a new user
    let query: Result<(i32,), _> = sqlx::query_as(r#"SELECT id FROM users WHERE email=$1"#)
        .bind(email.as_str())
        .fetch_one(&app_state.pool)
        .await;
    let user_id = if let Ok(query) = query {
        query.0
    } else {
        let query: (i32,) = sqlx::query_as(r#"INSERT INTO users (email) VALUES ($1) RETURNING id"#)
            .bind(email)
            .fetch_one(&app_state.pool)
            .await?;
        query.0
    };

    // Create a session for the user
    let session_token_p1 = Uuid::new_v4().to_string();
    let session_token_p2 = Uuid::new_v4().to_string();
    let session_token = [session_token_p1.as_str(), "_", session_token_p2.as_str()].concat();
    let headers = axum::response::AppendHeaders([(
        axum::http::header::SET_COOKIE,
        "session_token=".to_owned()
            + &*session_token
            + "; path=/; httponly; secure; samesite=strict",
    )]);
    let now = Utc::now().timestamp();

    sqlx::query(
        r#"INSERT INTO user_sessions
        (session_token_p1, session_token_p2, user_id, created_at, expires_at)
        VALUES ($1, $2, $3, $4, $5);"#,
    )
    .bind(session_token_p1)
    .bind(session_token_p2)
    .bind(user_id)
    .bind(now)
    .bind(now + 60 * 60 * 24)
    .execute(&app_state.pool)
    .await?;

    Ok((headers, Redirect::to(return_url.as_str())))
}
