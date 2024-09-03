use super::{copyright::copyright, scripts::scripts};
use maud::{html, Markup, DOCTYPE};

pub fn login_layout(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head lang="en" {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";

            title { (title) }

            link rel="stylesheet" href="/styles/login.css";

            (scripts())
        }
        body {
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
