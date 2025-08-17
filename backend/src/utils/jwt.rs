use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use std::env;
use crate::models;

#[derive( Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: usize,
}

static JWT_SECRET: Lazy<Vec<u8>> = Lazy::new(|| {
    env::var("JWT_SECRET").expect("JWT_SECRET must be set").into_bytes()
});

pub fn create_token(user: &models::user::User) -> jsonwebtoken::errors::Result<String> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims: Claims = Claims {
        sub: user.id.to_string(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(&JWT_SECRET))
}

pub fn decode_jwt(token: &str) -> jsonwebtoken::errors::Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(&JWT_SECRET),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}