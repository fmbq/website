use maud::{html, Markup};
use poem::{
    handler,
    session::Session,
    web::{Data, Form, Html, Query, Redirect},
    IntoResponse, Response,
};
use serde::Deserialize;
use sqlx::AnyPool;

use crate::{
    components::admin_layout::admin_layout,
    db::users::{self, LoginResult},
};

#[derive(Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
}

#[handler]
pub fn get() -> Html<Markup> {
    Html(admin_layout(
        "Log In",
        html! {
            form.login method="post" action="" {
                h1 { "Log in" }

                label for="email" { "Email" }
                input id="email" type="email" name="email" required;

                label for="password" { "Password" }
                input id="password" type="password" name="password" required;

                button type="submit" { "Log in" }
            }
        },
    ))
}

#[derive(Deserialize)]
pub struct LoginParams {
    redirect: Option<String>,
}

#[handler]
pub async fn submit(
    Data(db): Data<&AnyPool>,
    session: &Session,
    Form(f): Form<LoginForm>,
    Query(params): Query<LoginParams>,
) -> Response {
    let mut conn = db.acquire().await.unwrap();

    match users::validate_credentials(&mut conn, &f.email, &f.password)
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
        _ => Html(admin_layout(
            "Log In",
            html! {
                h1 { "Log in" }

                "Error"
            },
        ))
        .into_response(),
    }
}
