use axum::{
    extract::State, 
    Json, 
    http::StatusCode,
    response::IntoResponse
};
use tracing::{info, warn, error};
use crate::{models, router, services, utils};

pub async fn login(State(state): State<router::AppState>, Json(payload): Json<models::user::LoginPayload>) -> impl IntoResponse {
    match services::auth::auth(&state.db, &payload.email, &payload.password).await {
        Ok(user) => match utils::jwt::create_token(&user) {
            Ok(token) => {
                info!(action = "login", message = "User logged in successfully", user_id = user.id);
                Json(token).into_response()
            },
            Err(_) => {
                error!(action = "login", message = "Error creating JWT", user_id = "user_123");
                (StatusCode::UNAUTHORIZED, "Invalid credentials.".to_string()).into_response()
            },
        },
        Err(msg) => {
            warn!(action = "login", message = "Authentication failed", extra = format!("email = {}, msg = {}", payload.email, &msg));
            (StatusCode::UNAUTHORIZED, msg.to_string()).into_response()
        },
    }
}