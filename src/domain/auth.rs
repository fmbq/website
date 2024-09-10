use crate::{
    db::{
        users::{
            get_by_email,
            reset_password_token::{self, ResetPasswordToken},
            update_password_hash, update_require_password_reset, User,
        },
        Connection,
    },
    services::email::Mailer,
    web::components::email::email_layout,
};
use chrono::{TimeDelta, Utc};
use lettre::{
    message::{header::ContentType, Mailbox},
    Message,
};
use maud::html;
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

/// Request a password reset token for a user.
///
/// If a user for the given email address already exists, a new password reset
/// token will be created, allowing them to reset their password without knowing
/// their current password.
///
/// An email will be sent to that user's email address with a link containing the
/// reset token. Clicking the link will allow the user to "consume" the reset
/// token and set a new password.
pub async fn request_password_reset(
    connection: &mut Connection,
    mailer: &Mailer,
    email: &str,
) -> color_eyre::eyre::Result<()> {
    tracing::info!(email, "password reset requested");

    let mut tx = connection.begin().await?;

    let Some(user) = get_by_email(&mut tx, email).await else {
        // We don't give any indication that the email address doesn't exist. To
        // do otherwise would be a security vulnerability.
        tracing::info!(email, "password reset requested for unknown email address");

        return Ok(());
    };

    let reset_token = reset_password_token::create(&mut tx, &user.id).await?;

    tx.commit().await?;

    let email = create_password_reset_email(&user, &reset_token);
    mailer.send(email).await?;

    Ok(())
}

#[derive(Debug)]
pub enum ResetPasswordResult {
    Success,
    InvalidOrExpiredToken,
}

/// Reset a user's password to a new value given a reset token.
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

        // If the user was required to reset their password, then they've now
        // done so.
        update_require_password_reset(&mut tx, &token.user_id, false).await;

        tracing::info!(user_id = token.user_id, "password reset for user");

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

fn create_password_reset_email(user: &User, token: &str) -> Message {
    // TODO: Make hostname configurable.
    let reset_link = format!("http://localhost:5000/admin/reset-password?token={}", token);

    let html = email_layout(html! {
        p { "We received a request to reset your password. Click the link below to reset your password." }
        p { "If this was a mistake, just ignore this email and nothing will happen." }
        p {
            a href=(reset_link) { "Reset your password" }
        }
    });

    let sender = Mailbox::new(None, user.email.parse().unwrap());
    let destination = Mailbox::new(None, user.email.parse().unwrap());

    Message::builder()
        .from(sender)
        .to(destination)
        .subject("Password reset request")
        .header(ContentType::TEXT_HTML)
        .body(html.into_string())
        .unwrap()
}
