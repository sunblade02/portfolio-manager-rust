use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn connect_db() -> Result<PgPool, &'static str> {
    let database_url = match env::var("DATABASE_URL") {
        Ok(value) => value,
        Err(_) => return Err("DATABASE_URL must be set")
    };

    match PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await {
        Ok(pool) => Ok(pool),
        Err(_) => Err("Failed to connect to the database")
    }
} 