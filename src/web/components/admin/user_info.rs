use crate::db::users::User;
use maud::{html, Markup};

pub fn form(user: &User) -> Markup {
    html! {
        form hx-post="/admin/account/update-user-info" {
            h3 { "User Info" }

            .form-group {
                label for="first-name" { "First name" }
                input id="first-name" type="text" name="first_name" value=(user.first_name.as_deref().unwrap_or_default());
            }
            .form-group {
                label for="last-name" { "Last name" }
                input id="last-name" type="text" name="last_name" value=(user.last_name.as_deref().unwrap_or_default());
            }

            button type="submit" { "Save" }
        }
    }
}
