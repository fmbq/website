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
        }
        body {
            header {
                a href="/" { "Home" }

                .flex-end {
                    a href="/admin/login" { "Log In" }
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

            script src="/static/js/vendor/htmx/1.9.3/htmx.min.js" {}
            script src="/static/js/vendor/htmx/1.9.3/ext/sse.js" {}
        }
    }
}
