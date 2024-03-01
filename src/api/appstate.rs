use axum_macros::FromRef;
use sqlx::PgPool;

use crate::config::config::AppConfig;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub pool: PgPool,
    pub config: AppConfig,
}
