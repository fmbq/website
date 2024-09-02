use crate::{
    web::components::admin_layout::admin_layout,
    db::Pool,
    domain::users::{self, LoginResult},
};
use maud::{html, Markup};
use poem::{
    handler,
    session::Session,
    web::{Data, Form, Html, Query, Redirect},
    IntoResponse, Response,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
}

#[handler]
pub fn get() -> Html<Markup> {
    Html(crate::web::pages::login::login())
}

#[derive(Deserialize)]
pub struct LoginParams {
    redirect: Option<String>,
}

#[handler]
pub async fn submit(
    Data(db): Data<&Pool>,
    session: &Session,
    Form(form): Form<LoginForm>,
    Query(params): Query<LoginParams>,
) -> Response {
    let mut conn = db.acquire().await.unwrap();

    match users::login(&mut conn, &form.email, &form.password)
        .await
        .unwrap()
    {
        LoginResult::Success(id) => {
            session.set("user-id", id);

            if let Some(redirect) = params.redirect {
                Redirect::see_other(redirect).into_response()
            } else {
                Redirect::see_other("/").into_response()
            }
        }
        LoginResult::OkButPasswordResetRequired {
            user_id,
            reset_password_token,
        } => {
            session.set("user-id", user_id);

            Html(crate::web::pages::login::reset_password(
                Some(&reset_password_token),
                params.redirect.as_deref(),
            ))
            .into_response()
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
