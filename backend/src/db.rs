use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn connect_db() -> Result<PgPool, &'static str> {
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL must be set")?;

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|_| "Failed to connect to the database")
} 