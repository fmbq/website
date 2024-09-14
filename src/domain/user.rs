use super::auth::verify_user_password;
use crate::db::{
    users::{get_by_id, update_password_hash, User},
    Connection,
};
use password_auth::generate_hash;
use sqlx::Acquire;

pub async fn get_profile(connection: &mut Connection, id: &str) -> sqlx::Result<Option<User>> {
    Ok(get_by_id(connection, id).await)
}

/// Change a user's password. The current password must be provided to verify the
/// user's identity.
pub async fn change_password(
    connection: &mut Connection,
    id: &str,
    current_password: &str,
    new_password: &str,
) -> Result<(), ChangePasswordError> {
    let mut tx = connection.begin().await?;

    let Some(user) = get_by_id(&mut tx, id).await else {
        return Err(ChangePasswordError::UserNotFound(id.into()));
    };

    if !verify_user_password(&user, current_password) {
        return Err(ChangePasswordError::InvalidPassword);
    }

    update_password_hash(&mut tx, id, generate_hash(new_password)).await?;

    tx.commit().await?;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum ChangePasswordError {
    #[error("user not found with ID `{0}`")]
    UserNotFound(String),

    #[error("invalid password")]
    InvalidPassword,

    #[error("unknown database error")]
    Sql(#[from] sqlx::Error),
}
