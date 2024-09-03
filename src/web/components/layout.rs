use crate::web::components::scripts::scripts;
use maud::{html, Markup, DOCTYPE};
use time::OffsetDateTime;

pub fn layout(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head lang="en" {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";

            title { (title) }

            link rel="stylesheet" href="/styles/site.css";

            (scripts())
        }

        // Trigger all menus that may be open to close on losing focus.
        body _="on click send menuclose to <menu/>" {
            (crate::web::components::header::header())

            main role="main" class="content-grid" {
                (body)
            }

            footer {
                div class="container" {
                    p { a href="/api" { "API Documentation" } }
                    p { a href="/admin" { "Admin Area" } }
                    p { a href="/playground" { "Playground" } }
                    p class="center copyright" {
                        "© " (OffsetDateTime::now_utc().year()) " Free Methodist Bible Quizzing"
                    }
                }
            }
        }
    }
}
