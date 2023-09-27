use sqlx::{AnyConnection, FromRow};
use uuid::Uuid;

#[derive(FromRow, Debug)]
pub struct User {
    pub(crate) id: String,
    pub(crate) email: String,
    pub(crate) password_hash: String,

    #[sqlx(default)]
    require_password_reset: Option<i32>,
}

impl User {
    pub fn require_password_reset(&self) -> bool {
        self.require_password_reset == Some(1)
    }
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

pub async fn create(
    connection: &mut AnyConnection,
    email: &str,
    password: &str,
) -> Result<String, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let password_hash = password_auth::generate_hash(password);

    sqlx::query(
        "INSERT INTO user (id, email, password_hash, require_password_reset) VALUES (?, ?, ?, 0)",
    )
    .bind(&id)
    .bind(email)
    .bind(password_hash)
    .execute(connection)
    .await?;

    Ok(id)
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
) -> sqlx::Result<LoginResult> {
    if let Some(user) = get_by_email(connection, email).await {
        if user.require_password_reset() {
            return Ok(LoginResult::PasswordResetRequired);
        }

        match password_auth::verify_password(password, &user.password_hash) {
            Ok(()) => {
                // If password hash version is out of date, update it now.
                if password_auth::is_hash_obsolete(&user.password_hash) == Ok(true) {
                    sqlx::query("UPDATE user SET password_hash = ? WHERE id = ?")
                        .bind(password_auth::generate_hash(password))
                        .bind(&user.id)
                        .execute(connection)
                        .await?;
                }

                Ok(LoginResult::Success(user.id.clone()))
            }
            Err(_) => Ok(LoginResult::InvalidCredentials),
        }
    } else {
        Ok(LoginResult::UnknownUser)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn login_reset() {
        let email = "foo@example.org";
        let mut db = crate::db::create_test_connection().await;

        assert!(get_by_email(&mut db, email).await.is_none());

        let id = create(&mut db, email, "test").await.unwrap();

        let user = get_by_id(&mut db, &id).await.unwrap();
        assert_eq!(user.id, id);

        let user = get_by_email(&mut db, email).await.unwrap();
        assert_eq!(user.id, id);
    }
}
