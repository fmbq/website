use super::{copyright::copyright, header::header, scripts::scripts};
use maud::{html, Markup, DOCTYPE};

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
            (header())

            main role="main" class="content-grid" {
                (body)
            }

            footer {
                div class="container" {
                    p { a href="/api" { "API Documentation" } }
                    p { a href="/admin" { "Admin Area" } }
                    p { a href="/playground" { "Playground" } }

                    (copyright())
                }
            }
        }
    }
}
