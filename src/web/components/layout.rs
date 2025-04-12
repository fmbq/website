use super::{footer::footer, header::header, scripts::scripts};
use maud::{html, Markup, DOCTYPE};

const GLOBAL_TITLE: &str = "Free Methodist Bible Quizzing";

pub fn layout(title: impl AsRef<str>, body: Markup) -> Markup {
    let title = title.as_ref();
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

        body {
            (header())

            main role="main" class="content-grid" {
                (body)
            }

            (footer())
        }
    }
}
