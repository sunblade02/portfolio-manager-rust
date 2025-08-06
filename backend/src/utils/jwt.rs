use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use std::env;

#[derive( Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: usize,
}

static JWT_SECRET: Lazy<Vec<u8>> = Lazy::new(|| {
    env::var("JWT_SECRET").expect("JWT_SECRET must be set").into_bytes()
});

pub fn create_token(user_id: &str) -> jsonwebtoken::errors::Result<String> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(&JWT_SECRET))
}