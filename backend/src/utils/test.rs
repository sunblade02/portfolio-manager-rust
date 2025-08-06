use axum::Router;
use sqlx::PgPool;
use dotenvy::from_filename;
use std::fs;
use crate::{router, db};

// execute all SQL queries in dataset.sql on the TEST environment
async fn init_dataset(db: &PgPool) {
    let content = fs::read_to_string("tests/dataset.sql").expect("Failed to open dataset file");

    let mut tx = db.begin().await.unwrap();
    for stmt in content.split(';') {
        if !stmt.is_empty() {
            sqlx::query(stmt)
                .execute(&mut *tx)
                .await
                .expect("Failed to load dataset");
        }
    }
    tx.commit().await.unwrap();
}

// initialyze the router for the TEST environment
pub async fn init_app() -> Router {
    from_filename(".env.test").ok();
    let db = db::connect_db().await.expect("Failed to connect to DB");

    init_dataset(&db).await;

    router::create_app(router::AppState {
        db: db.clone().into(),
    })
}