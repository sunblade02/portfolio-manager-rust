use dotenvy::{dotenv, from_filename};
use sqlx::{PgPool, migrate::Migrator};
use std::{env, path::Path};

// execute SQL scripts to update the database

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 && args[1] == "test" {
        from_filename(".env.test").ok();
    } else {
        dotenv().ok();
    }

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url).await?;

    println!("Applying migrations...");
    let migrator = Migrator::new(Path::new("./migrations")).await?;
    migrator.run(&pool).await?;
    println!("Migrations applied.");

    Ok(())
}