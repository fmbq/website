use crate::{
    db::{
        users::{get_by_email, User},
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

pub async fn get_profile(connection: &mut Connection) {}
