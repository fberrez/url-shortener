use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub postgres_uri: String,
    pub oauth2_google_client_id: String,
    pub oauth2_google_client_secret: String,
    pub oauth2_google_redirect_url: String,
}

pub fn config() -> Result<AppConfig, ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::with_name("./Settings.toml"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()?;

    let config = settings.try_deserialize::<AppConfig>()?;
    Ok(config)
}
