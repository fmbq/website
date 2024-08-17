use crate::db::Connection;
use sqlx::{
    types::chrono::{DateTime, Utc},
    FromRow,
};
use uuid::Uuid;

#[derive(FromRow, Debug)]
pub struct ResetPasswordToken {
    pub(crate) user_id: String,
    pub(crate) token: String,
    pub(crate) created_time: DateTime<Utc>,
}

/// Generate a new reset password token for a user.
pub async fn create(connection: &mut Connection, user_id: &str) -> Result<String, sqlx::Error> {
    let token = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO reset_password_token (user_id, token, created_time) VALUES (?, ?, ?)")
        .bind(user_id)
        .bind(&token)
        .bind(Utc::now())
        .execute(connection)
        .await?;

    Ok(token)
}

pub async fn get_by_token(
    connection: &mut Connection,
    token: &str,
) -> Result<Option<ResetPasswordToken>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM reset_password_token WHERE token = ?")
        .bind(token)
        .fetch_optional(connection)
        .await
}

pub async fn delete(connection: &mut Connection, token: &str) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM reset_password_token WHERE token = ?")
        .bind(token)
        .execute(connection)
        .await?;

    Ok(result.rows_affected() == 1)
}
