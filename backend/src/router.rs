use axum::{Router, routing::post};
use http::HeaderValue;
use sqlx::PgPool;
use std::env;
use tower_http::cors::{CorsLayer, Any};
use crate::handlers;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub fn create_app(state: AppState) -> Router {
    let cors_allow_origin = env::var("CORS_ALLOW_ORIGIN").unwrap_or_else(|_err| "http://localhost:4200".to_string());

    let header_value = HeaderValue::from_str(&cors_allow_origin)
        .expect("Invalid CORS_ALLOW_ORIGIN");

    let cors = CorsLayer::new()
        .allow_origin(header_value)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/auth/login", post(handlers::auth::login))
        .layer(cors)
        .with_state(state)
}