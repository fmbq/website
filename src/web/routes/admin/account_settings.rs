use crate::{
    db::Pool,
    domain::user::change_password,
    web::{
        login_context::LoginContext,
        pages::admin::{AccountSettings, AdminModule},
    },
};
use maud::Markup;
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Form, Html},
};
use serde::Deserialize;

#[derive(Deserialize)]
struct ChangePasswordForm {
    current_password: String,
    new_password: String,
}

#[handler]
pub async fn get(login_context: LoginContext) -> Html<Markup> {
    Html(AccountSettings.render(&login_context))
}

#[handler]
pub async fn submit_change_password(
    login_context: LoginContext,
    Data(db): Data<&Pool>,
    Form(form): Form<ChangePasswordForm>,
) -> StatusCode {
    let mut conn = db.acquire().await.unwrap();

    change_password(
        &mut conn,
        &login_context.user_id,
        &form.current_password,
        &form.new_password,
    )
    .await
    .unwrap();

    StatusCode::OK
}
