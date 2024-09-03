use crate::web::components::{
    admin::password_reset::{request_password_reset_form, reset_password_form},
    login_layout::login_layout,
};
use maud::{html, Markup};

pub fn login() -> Markup {
    login_layout(
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
    )
}

pub fn request_password_reset() -> Markup {
    login_layout(
        "Reset Password",
        html! {
            (request_password_reset_form())
        },
    )
}

pub fn reset_password(token: Option<&str>, redirect: Option<&str>) -> Markup {
    login_layout(
        "Reset Password",
        html! {
            (reset_password_form(token, redirect))
        },
    )
}
