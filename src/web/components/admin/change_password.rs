use maud::{html, Markup};

pub fn form() -> Markup {
    html! {
        form hx-post="/admin/account/change-password" {
            h3 { "Change Password" }

            .form-group {
                label for="current-password" { "Current password" }
                input id="current-password" type="password" name="current_password" required;
            }
            .form-group {
                label for="password" { "New password" }
                input id="password" type="password" name="new_password" required;
            }
            .form-group {
                label for="password-confirmation" { "Retype new password" }
                input id="password-confirmation" type="password" name="new_password_confirmation" required;
            }

            button type="submit" { "Save" }
        }
    }
}
