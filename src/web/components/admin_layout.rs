use crate::web::components::scripts::scripts;
use maud::{html, Markup, DOCTYPE};
use time::OffsetDateTime;

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
                a href="/" { "Home" }

                .flex-end {
                    a href="/admin/logout" { "Log Out" }
                }
            }

            main role="main" {
                (body)
            }

            footer {
                div class="container" {
                    p class="center copyright" {
                        "© " (OffsetDateTime::now_utc().year()) " Free Methodist Bible Quizzing"
                    }
                }
            }
        }
    }
}
