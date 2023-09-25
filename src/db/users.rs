use sqlx::{AnyConnection, FromRow, Row};

#[derive(FromRow, Debug)]
pub struct User {
    id: String,
    email: String,
    password_hash: String,
    require_password_reset: bool,
}

#[derive(Debug)]
pub enum LoginResult {
    Success(String),
    InvalidCredentials,
    UnknownUser,
    PasswordResetRequired,
}

pub async fn get_by_id(connection: &mut AnyConnection, id: &str) -> Option<User> {
    sqlx::query_as("SELECT * FROM user WHERE id = ?")
        .bind(id)
        .fetch_optional(connection)
        .await
        .unwrap()
}

pub async fn get_by_email(connection: &mut AnyConnection, email: &str) -> Option<User> {
    sqlx::query_as("SELECT * FROM user WHERE email = ?")
        .bind(email)
        .fetch_optional(connection)
        .await
        .unwrap()
}

pub async fn reset_password(connection: &mut AnyConnection, id: &str) -> bool {
    sqlx::query("UPDATE user SET require_password_reset = 1 WHERE id = ?")
        .bind(id)
        .execute(connection)
        .await
        .unwrap()
        .rows_affected()
        > 0
}

pub async fn validate_credentials(
    connection: &mut AnyConnection,
    email: &str,
    password: &str,
) -> LoginResult {
    if let Some(user) = get_by_email(connection, email).await {
        if user.require_password_reset {
            return LoginResult::PasswordResetRequired;
        }

        match password_auth::verify_password(password, &user.password_hash) {
            Ok(()) => LoginResult::Success(user.id.clone()),
            Err(_) => LoginResult::InvalidCredentials,
        }
    } else {
        LoginResult::UnknownUser
    }
}
