use maud::{html, Markup};

pub fn header() -> Markup {
    html! {
        header {
            nav hx-boost="true" {
                div {
                    a id="logo" class="title" href="/" { "Free Methodist Bible Quizzing" }
                }

                div.menu-container {
                    span.menu-toggle _="on click toggle .open on <menu/> then halt the event" { "[Menu]" }

                    menu _="on menuclose remove .open on me" {
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
    }
}
