use super::{admin::account_menu::account_menu, copyright::copyright, scripts::scripts};
use maud::{html, Markup, DOCTYPE};

pub fn admin_layout(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head lang="en" {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";

            title { (title) }

            link rel="stylesheet" href="/styles/admin.css";

            (scripts())
        }
        body {
            header {
                a href="/admin" { "Home" }

                .flex-end {
                    (account_menu())
                }
            }

            main role="main" {
                (body)
            }

            footer {
                div class="container" {
                    (copyright())
                }
            }
        }
    }
}
