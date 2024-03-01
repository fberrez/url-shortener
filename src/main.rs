use axum::http::{HeaderValue, Method};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use url_shortener::api::appstate::AppState;
use url_shortener::api::router::create_router;
use url_shortener::config;

#[tokio::main]
async fn main() {
    let config = config::config::config().unwrap();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::DELETE]);
    let pool = match PgPoolOptions::new()
        .max_connections(10) // Adjust as needed
        .connect(&config.postgres_uri)
        .await
    {
        Ok(pool) => pool,
        Err(e) => panic!("Failed to connect to Postgres: {}", e),
    };

    let app = create_router(AppState { pool, config }).layer(cors);

    let addr = "0.0.0.0:3000";

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(e) => panic!("Failed to bind to address: {}", e),
    };

    println!("🚀 Listening on http://{}", addr);
    match axum::serve(listener, app).await {
        Ok(_) => {}
        Err(e) => panic!("Failed to start server: {}", e),
    }
}
