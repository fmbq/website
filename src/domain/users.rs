use crate::db::{
    users::{
        get_by_email,
        reset_password_token::{self, ResetPasswordToken},
        update_password_hash,
    },
    Connection,
};
use chrono::{TimeDelta, Utc};
use password_auth::{generate_hash, is_hash_obsolete, verify_password};
use sqlx::Acquire;

const RESET_TOKEN_EXPIRATION: TimeDelta = TimeDelta::hours(4);

#[derive(Debug)]
pub enum LoginResult {
    Success(String),
    OkButPasswordResetRequired {
        user_id: String,
        reset_password_token: String,
    },
    InvalidCredentials,
    UnknownUser,
}

/// Log in a user given their email and password.
pub async fn login(
    connection: &mut Connection,
    email: &str,
    password: &str,
) -> sqlx::Result<LoginResult> {
    let mut tx = connection.begin().await?;

    let Some(user) = get_by_email(&mut tx, email).await else {
        return Ok(LoginResult::UnknownUser);
    };

    if verify_password(password, &user.password_hash).is_err() {
        return Ok(LoginResult::InvalidCredentials);
    }

    let mut require_commit = false;

    // If password hash version is out of date, update it now since we have the
    // user's raw password.
    if is_hash_obsolete(&user.password_hash) == Ok(true) {
        update_password_hash(&mut tx, &user.id, generate_hash(password)).await?;
        require_commit = true;
    }

    let reset_password_token = if user.require_password_reset() {
        let reset_token = reset_password_token::create(&mut tx, &user.id).await?;

        require_commit = true;

        Some(reset_token)
    } else {
        None
    };

    if require_commit {
        tx.commit().await?;
    }

    if let Some(reset_password_token) = reset_password_token {
        Ok(LoginResult::OkButPasswordResetRequired {
            user_id: user.id,
            reset_password_token,
        })
    } else {
        Ok(LoginResult::Success(user.id))
    }
}

#[derive(Debug)]
pub enum ResetPasswordResult {
    Success,
    InvalidOrExpiredToken,
}

/// Reset a user's password given a reset token.
pub async fn reset_password(
    connection: &mut Connection,
    token: &str,
    new_password: &str,
) -> sqlx::Result<ResetPasswordResult> {
    let mut tx = connection.begin().await?;

    let Some(token) = reset_password_token::get_by_token(&mut tx, token).await? else {
        return Ok(ResetPasswordResult::InvalidOrExpiredToken);
    };

    let expired = if token.is_expired() {
        true
    } else {
        // Apply the new password.
        update_password_hash(&mut tx, &token.user_id, generate_hash(new_password)).await?;

        false
    };

    // Once the token is used, it is invalid.
    reset_password_token::delete(&mut tx, &token.token).await?;

    tx.commit().await?;

    if expired {
        Ok(ResetPasswordResult::InvalidOrExpiredToken)
    } else {
        Ok(ResetPasswordResult::Success)
    }
}

impl ResetPasswordToken {
    /// Check if the token is expired.
    pub fn is_expired(&self) -> bool {
        self.created_time + RESET_TOKEN_EXPIRATION < Utc::now()
    }
}
