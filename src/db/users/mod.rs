use crate::db::Connection;
use sqlx::FromRow;
use uuid::Uuid;

pub mod reset_password_token;

#[derive(FromRow, Debug)]
pub struct User {
    pub(crate) id: String,
    pub(crate) email: String,
    pub(crate) first_name: Option<String>,
    pub(crate) last_name: Option<String>,
    pub(crate) password_hash: String,

    #[sqlx(default)]
    require_password_reset: Option<i32>,
}

impl User {
    pub fn display_name(&self) -> String {
        if let Some(first_name) = &self.first_name {
            if let Some(last_name) = &self.last_name {
                format!("{} {}", first_name, last_name)
            } else {
                first_name.clone()
            }
        } else {
            self.email.clone()
        }
    }

    pub fn require_password_reset(&self) -> bool {
        self.require_password_reset == Some(1)
    }
}

pub async fn get_by_id(connection: &mut Connection, id: &str) -> Option<User> {
    sqlx::query_as("SELECT * FROM user WHERE id = ?")
        .bind(id)
        .fetch_optional(connection)
        .await
        .unwrap()
}

pub async fn get_by_email(connection: &mut Connection, email: &str) -> Option<User> {
    sqlx::query_as("SELECT * FROM user WHERE email = ?")
        .bind(email)
        .fetch_optional(connection)
        .await
        .unwrap()
}

pub async fn update_info(
    connection: &mut Connection,
    id: &str,
    first_name: &str,
    last_name: &str,
) -> sqlx::Result<bool> {
    let result = sqlx::query("UPDATE user SET first_name = ?, last_name = ? WHERE id = ?")
        .bind(first_name)
        .bind(last_name)
        .bind(id)
        .execute(connection)
        .await?;

    Ok(result.rows_affected() == 1)
}

pub async fn update_password_hash(
    connection: &mut Connection,
    id: &str,
    password_hash: String,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE user SET password_hash = ? WHERE id = ?")
        .bind(password_hash)
        .bind(id)
        .execute(connection)
        .await?;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum CreateAccountError {
    #[error("email address is already in use")]
    EmailAlreadyInUse,
    #[error("unknown database error")]
    Sql(#[from] sqlx::Error),
}

pub async fn create(
    connection: &mut Connection,
    email: &str,
    password: &str,
) -> Result<String, CreateAccountError> {
    let id = Uuid::new_v4().to_string();
    let password_hash = password_auth::generate_hash(password);

    match sqlx::query(
        "INSERT INTO user (id, email, password_hash, require_password_reset) VALUES (?, ?, ?, 0)",
    )
    .bind(&id)
    .bind(email)
    .bind(password_hash)
    .execute(connection)
    .await
    {
        Ok(_) => Ok(id),
        Err(sqlx::Error::Database(e)) if e.is_unique_violation() => {
            Err(CreateAccountError::EmailAlreadyInUse)
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn update_require_password_reset(
    connection: &mut Connection,
    id: &str,
    require: bool,
) -> bool {
    sqlx::query("UPDATE user SET require_password_reset = ? WHERE id = ?")
        .bind(i64::from(require))
        .bind(id)
        .execute(connection)
        .await
        .unwrap()
        .rows_affected()
        > 0
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
