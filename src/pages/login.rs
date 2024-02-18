use crate::components::admin_layout::admin_layout;
use maud::{html, Markup};

pub fn login() -> Markup {
    admin_layout(
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

pub fn reset_password() -> Markup {
    admin_layout(
        "Reset Password",
        html! {
            form.login method="post" action="" {
                h1 { "Reset Password" }

                p { "You must set a new password to continue to your account." }

                label for="password" { "Password" }
                input id="password" type="password" name="password" required;

                label for="retype-password" { "Retype Password" }
                input id="retype-password" type="password" name="retype-password" required;

                button type="submit" { "Continue" }
            }
        },
    )
}
