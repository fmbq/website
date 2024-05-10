use maud::{html, Markup};

pub fn header() -> Markup {
    html! {
        header {
            nav hx-boost="true" {
                a id="logo" class="title" href="/" { "Free Methodist Bible Quizzing" }
                a href="/about" {
                    span { "About" }
                }
                a href="/news" {
                    span { "News" }
                }
                a href="/get-started" {
                    span { "Get Started" }
                }
                a href="/resources" {
                    span { "Resources" }
                }
                a href="/contact" {
                    span { "Contact" }
                }
            }
        }
    }
}
