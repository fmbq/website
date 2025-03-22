use super::{copyright::copyright, header::header, scripts::scripts};
use maud::{html, Markup, DOCTYPE};

const GLOBAL_TITLE: &str = "Free Methodist Bible Quizzing";

pub fn layout(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head lang="en" {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";

            link rel="icon" href="/favicon.ico" sizes="32x32";
            link rel="apple-touch-icon" href="/apple-touch-icon.png";

            title {
                @if title == GLOBAL_TITLE {
                    (title)
                } @else {
                    (format!("{title} – {GLOBAL_TITLE}"))
                }
            }

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
                    (copyright())
                }
            }
        }
    }
}
