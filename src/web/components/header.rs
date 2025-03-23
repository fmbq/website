use maud::{html, Markup};

pub fn header() -> Markup {
    html! {
        // Trigger all menus that may be open to close on losing focus.
        header #site-header role="banner" _="on click from document send close to #main-menu" {
            nav hx-boost="true" {
                a id="logo" class="title" href="/" {
                    img src="/static/resources/logos/FMBQ-logo.png";

                    "Free Methodist Bible Quizzing"
                }

                menu #main-menu _="
                    on close if I match .open then add .closing then remove .open end
                    on toggle if I match .open then send close else add .open end
                    on animationend remove .closing
                "{
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
                    a href="/support-us" {
                        span { "Support Us" }
                    }
                    a href="/contacts" {
                        span { "Contacts" }
                    }
                }

                menu #mobile-menu {
                    a _="on click send toggle to #main-menu then halt the event" {
                        svg role="img" {
                            line x1="0%" y1="25%" x2="100%" y2="25%" {}
                            line x1="0%" y1="50%" x2="100%" y2="50%" {}
                            line x1="0%" y1="75%" x2="100%" y2="75%" {}
                        }
                    }
                }
            }
        }
    }
}
