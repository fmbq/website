use maud::{html, Markup};
use poem::{
    handler,
    web::{Data, Form, Html, Redirect},
    IntoResponse, Response,
};
use serde::Deserialize;
use sqlx::AnyPool;

use crate::db::users::{self, LoginResult};

#[derive(Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
}

#[handler]
pub fn get() -> Html<Markup> {
    Html(html! {
        h1 { "Log in" }

        form method="post" action="/login" {
            label for="email" { "Email" }
            input id="email" type="email" name="email" required;

            label for="password" { "Password" }
            input id="password" type="password" name="password" required;

            button type="submit" { "Log in" }
        }
    })
}

#[handler]
pub async fn submit(Data(db): Data<&AnyPool>, Form(f): Form<LoginForm>) -> Response {
    let mut conn = db.acquire().await.unwrap();

    match users::validate_credentials(&mut conn, &f.email, &f.password).await {
        LoginResult::Success(_id) => Redirect::temporary("/").into_response(),
        _ => Html(html! {
            h1 { "Log in" }

            "Error"
        })
        .into_response(),
    }
}
