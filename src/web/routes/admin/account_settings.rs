use crate::{
    db::Pool,
    domain::user::{change_password, update_info},
    web::{
        login_context::LoginContext,
        pages::admin::{AccountSettings, AdminModule},
    },
};
use maud::Markup;
use poem::{
    handler,
    web::{Data, Form},
};
use poem_htmx::responses::Refresh;
use serde::Deserialize;

#[derive(Deserialize)]
struct UpdateUserInfoForm {
    first_name: String,
    last_name: String,
}

#[derive(Deserialize)]
struct ChangePasswordForm {
    current_password: String,
    new_password: String,
}

#[handler]
pub async fn get(login_context: LoginContext) -> Markup {
    AccountSettings.render(&login_context)
}

#[handler]
pub async fn submit_change_password(
    login_context: LoginContext,
    Data(db): Data<&Pool>,
    Form(form): Form<ChangePasswordForm>,
) -> Refresh {
    let mut conn = db.acquire().await.unwrap();

    change_password(
        &mut conn,
        &login_context.user_id,
        &form.current_password,
        &form.new_password,
    )
    .await
    .unwrap();

    Refresh
}

#[handler]
pub async fn update_user_info(
    login_context: LoginContext,
    Data(db): Data<&Pool>,
    Form(form): Form<UpdateUserInfoForm>,
) -> Refresh {
    let mut conn = db.acquire().await.unwrap();

    update_info(
        &mut conn,
        &login_context.user_id,
        &form.first_name,
        &form.last_name,
    )
    .await
    .unwrap();

    Refresh
}
