use sqlx::PgPool;
use crate::{services, utils};

// authenticate a user by fetching data from the database and checking password (bcrypt)
pub async fn auth(db: &PgPool, email: &String, password: &String) -> Result<(), &'static str> {
    let hashed_password = match services::user::get_password_by_email(db, email).await {
        Ok(password) => password,
        Err(sqlx::Error::RowNotFound) => return Err("Invalid credentials"),
        Err(_) => return Err("System error"),
    };

    if utils::password::verify_password(&hashed_password, password).unwrap() {
        Ok(())
    } else {
        Err("Invalid credentials")
    }
}