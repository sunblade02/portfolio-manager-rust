use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String
}