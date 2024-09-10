use crate::{
    db::Pool, domain::auth::reset_password,
    web::components::admin::password_reset::password_reset_complete,
};
use maud::Markup;
use poem::{
    handler,
    web::{Data, Form, Html, Query},
    IntoResponse, Response,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ResetPasswordForm {
    /// The reset token, which is required for resetting the password.
    token: String,
    password: String,
    password_confirmation: String,
    redirect: Option<String>,
}

#[derive(Deserialize)]
pub struct ResetPasswordParams {
    /// The reset token, which is required for resetting the password.
    token: String,

    redirect: Option<String>,
}

#[handler]
pub fn get(Query(params): Query<ResetPasswordParams>) -> Html<Markup> {
    Html(crate::web::pages::login::reset_password(
        Some(&params.token),
        params.redirect.as_deref(),
    ))
}

#[handler]
pub async fn submit(Data(db): Data<&Pool>, Form(form): Form<ResetPasswordForm>) -> Response {
    let mut conn = db.acquire().await.unwrap();

    // TODO: Make sure the user typed the same password both times.

    reset_password(&mut conn, &form.token, &form.password)
        .await
        .unwrap();

    Html(password_reset_complete()).into_response()
}
