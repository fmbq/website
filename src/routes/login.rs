use maud::{html, Markup};
use poem::{
    handler,
    session::Session,
    web::{Data, Form, Html, Query, Redirect},
    IntoResponse, Response,
};
use serde::Deserialize;

use crate::{
    components::admin_layout::admin_layout,
    db::Pool,
    domain::users::{self, LoginResult},
};

#[derive(Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct ResetPasswordForm {
    password: String,
    password_confirmation: String,
}

#[handler]
pub fn get() -> Html<Markup> {
    Html(crate::pages::login::login())
}

#[derive(Deserialize)]
pub struct LoginParams {
    redirect: Option<String>,
}

#[handler]
pub async fn submit(
    Data(db): Data<&Pool>,
    session: &Session,
    Form(f): Form<LoginForm>,
    Query(params): Query<LoginParams>,
) -> Response {
    let mut conn = db.acquire().await.unwrap();

    match users::login(&mut conn, &f.email, &f.password)
        .await
        .unwrap()
    {
        LoginResult::Success(id) => {
            session.set("user-id", id);

            if let Some(redirect) = params.redirect {
                Redirect::temporary(redirect).into_response()
            } else {
                Redirect::temporary("/").into_response()
            }
        }
        result => Html(admin_layout(
            "Log In",
            html! {
                h1 { "Log in" }

                (format!("Error: {:?}", result))
            },
        ))
        .into_response(),
    }
}

#[derive(Deserialize)]
pub struct ResetPasswordParams {
    redirect: Option<String>,
}

#[handler]
pub async fn submit_reset_password(
    Data(db): Data<&Pool>,
    session: &Session,
    Form(f): Form<ResetPasswordForm>,
    Query(params): Query<ResetPasswordParams>,
) -> Response {
    let mut conn = db.acquire().await.unwrap();

    todo!()
}
