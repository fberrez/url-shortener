use axum::{http::StatusCode, response::IntoResponse};
use serde_json::json;

pub struct AppError {
    code: StatusCode,
    message: String,
    user_message: String,
}

impl AppError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            user_message: "".to_owned(),
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    pub fn with_user_message(self, user_message: impl Into<String>) -> Self {
        Self {
            user_message: user_message.into(),
            ..self
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let body = json!({
            "message": self.message,
            "user_message": self.user_message,
        });
        (
            self.code,
            axum::http::HeaderMap::new(),
            serde_json::to_vec(&body).unwrap(),
        )
            .into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::new(format!("Database query error: {:#}", err))
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::new(err)
    }
}

impl From<&str> for AppError {
    fn from(err: &str) -> Self {
        AppError::new(err)
    }
}
