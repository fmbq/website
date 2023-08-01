use maud::{html, Markup, DOCTYPE};
use time::OffsetDateTime;

pub mod home;

fn layout(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head lang="en" {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";

            title { (title) }

            link rel="stylesheet" href="/styles/site.css";
            script defer src="/js/vendor/htmx/1.9.3/htmx.min.js" {}
            script defer src="/js/vendor/htmx/1.9.3/ext/sse.js" {}
        }
        body {
            header {
                nav hx-boost="true" {
                    a id="logo" class="title" href="/" { "Free Methodist Bible Quizzing" }
                    div class="links" {
                        a class="button" href="/about" { "About" }
                        a class="button" href="/media" { "Media" }
                        a class="button" href="/finals" { "Finals" }
                    }
                }
            }

            main role="main" {
                (body)
            }

            footer {
                div class="container" {
                    a href="/api" { "API Documentation" }
                    p class="center copyright" {
                        "© " (OffsetDateTime::now_utc().year()) " Free Methodist Bible Quizzing"
                    }
                }
            }
        }
    }
}

pub fn about() -> Markup {
    layout(
        "About",
        html! {
            h1 { "About" }
            h2 { "What is Quizzing?" }

            p {
                "Bible Quizzing is a ministry that encourages 6th through 12th grade students to
                study and memorize Scripture through competition. Each year a specific portion of
                scripture is chosen (1 & 2 Corinthians, John, Romans & James) and quizzers will
                study that text in weekly increments throughout the school year. The competition
                comes when quizzers are matched with other quizzers to see who can answer the most
                questions in a series of 15-question quiz rounds. This is done on a weekly basis
                during Quiz practice, monthly at local tournaments, and annually at
                Conference/Regional quizzes and Quiz Finals."
            }

            p {
                "Free Methodist Bible Quizzing uses the NIV-2011 translation of the Holy Bible. Our
                current season (2021-2022) is over " b { "1&2 Timothy, Titus, Matthew 1-13" } "."
            }
        },
    )
}
