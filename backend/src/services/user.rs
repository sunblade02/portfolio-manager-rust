use sqlx::{Error, PgPool};
use crate::models;

pub async fn find_by_email(db: &PgPool, email: &String) -> Result<models::user::User, Error> {
    let row = sqlx::query!(
            r#"
            SELECT id, email, password
            FROM "user"
            WHERE email=$1
            "#,
            email
        ).fetch_one(db)
        .await?;

    Ok(models::user::User {
        id: row.id.to_string(),
        email: row.email,
        password: row.password,
    })
}