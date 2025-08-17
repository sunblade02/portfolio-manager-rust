use sqlx::PgPool;
use crate::{models, services, utils};

// authenticate a user by fetching data from the database and checking password (bcrypt)
pub async fn auth(db: &PgPool, email: &String, password: &String) -> Result<models::user::User, &'static str> {
    let user = match services::user::find_by_email(db, email).await {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => return Err("Invalid credentials"),
        Err(_) => return Err("System error"),
    };

    let hashed_password = &user.password;

    if utils::password::verify_password(&hashed_password, password).unwrap() {
        Ok(user)
    } else {
        Err("Invalid credentials")
    }
}