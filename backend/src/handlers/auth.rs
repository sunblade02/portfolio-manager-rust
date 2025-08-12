use axum::{
    extract::State, 
    Json, 
    http::StatusCode,
    response::IntoResponse
};
use crate::{models, router, services, utils};

pub async fn login(State(state): State<router::AppState>, Json(payload): Json<models::user::LoginPayload>) -> impl IntoResponse {
    match services::auth::auth(&state.db, &payload.email, &payload.password).await {
        Ok(_) => match utils::jwt::create_token(&payload.email) {
            Ok(token) => Json(token).into_response(),
            Err(_) => (StatusCode::UNAUTHORIZED, "Invalid credentials.".to_string()).into_response(),
        },
        Err(msg) => (StatusCode::UNAUTHORIZED, msg.to_string()).into_response(),
    }
}