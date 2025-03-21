use crate::{db::users, db::Pool, web::components::login_layout::login_layout};
use maud::{html, Markup};
use poem::{
    handler,
    web::{Data, Form},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateAccountForm {
    email: String,
    password: String,
}

#[handler]
pub fn get() -> Markup {
    login_layout(
        "Create Account",
        html! {
            form.login method="post" action="" {
                h1 { "Create Account" }

                label for="email" { "Email" }
                input id="email" type="email" name="email" required;

                label for="password" { "Password" }
                input id="password" type="password" name="password" required;

                button type="submit" { "Create Account" }
            }
        },
    )
}

#[handler]
pub async fn submit(Data(db): Data<&Pool>, Form(f): Form<CreateAccountForm>) -> Markup {
    let mut conn = db.acquire().await.unwrap();

    users::create(&mut conn, &f.email, &f.password)
        .await
        .unwrap();

    login_layout(
        "Create Account",
        html! {
            h1 { "Create Account" }

            "We have received your request. Once your account is approved you may log in."
        },
    )
}
