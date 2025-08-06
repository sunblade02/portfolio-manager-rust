use sqlx::{Error, PgPool};

pub async fn get_password_by_email(db: &PgPool, email: &String) -> Result<String, Error> {
    let row = sqlx::query!(
            r#"
            SELECT password
            FROM "user"
            WHERE email=$1
            "#,
            email
        ).fetch_one(db)
        .await?;

    Ok(row.password)
}