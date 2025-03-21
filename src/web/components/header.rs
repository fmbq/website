use maud::{html, Markup};

pub fn header() -> Markup {
    html! {
        header role="banner" {
            nav hx-boost="true" {
                div {
                    a id="logo" class="title" href="/" {
                        img src="/static/resources/logos/FMBQ-logo.png";

                        "Free Methodist Bible Quizzing"
                    }
                }

                div.menu-container {
                    span.menu-toggle _="on click toggle .open on <menu/> then halt the event" { "[Menu]" }

                    menu _="on menuclose remove .open on me" {
                        a href="/about" {
                            span { "About" }
                        }
                        a href="/schedule" {
                            span { "Schedule" }
                        }
                        a href="/resources" {
                            span { "Resources" }
                        }
                        a href="/awards" {
                            span { "Awards" }
                        }
                        a href="/support_us" {
                            span { "Support Us" }
                        }
                        a href="/contacts" {
                            span { "Contacts" }
                        }
                    }
                }
            }
        }
    }
}
