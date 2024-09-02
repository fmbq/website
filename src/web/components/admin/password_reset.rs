use maud::{html, Markup};

pub fn request_password_reset_form() -> Markup {
    html! {
        form.login hx-post="/admin/request-password-reset" {
            h1 { "Reset Password" }

            label for="email" { "Email" }
            input id="email" type="email" name="email" required;

            button type="submit" { "Reset Password" }
        }
    }
}

pub fn request_password_reset_accepted() -> Markup {
    html! {
        p {
            "An email has been sent to the address you provided. Please check your email for instructions on how to reset your password."
        }
    }
}

/// A form allowing the user to reset their password.
///
/// A redirect location may be optionally included in the form. If reset
/// successfully, the user will be redirected to this location.
pub fn reset_password_form(token: Option<&str>, redirect: Option<&str>) -> Markup {
    html! {
        form.login hx-post="/admin/reset-password" {
            h1 { "Reset Password" }

            p { "You must set a new password to continue to your account." }

            input type="hidden" name="token" value=(token.unwrap_or_default());
            input type="hidden" name="redirect" value=(redirect.unwrap_or_default());

            label for="password" { "Password" }
            input id="password" type="password" name="password" required;

            label for="password-confirmation" { "Retype Password" }
            input id="password-confirmation" type="password" name="password_confirmation" required;

            button type="submit" { "Continue" }
        }
    }
}

pub fn password_reset_complete() -> Markup {
    html! {
        p {
            "Your password has been reset. You may now log in with your new password."
        }
        p {
            a href="/admin/login" { "Log in" }
        }
    }
}
