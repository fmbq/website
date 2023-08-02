use maud::{html, Markup};

pub fn header() -> Markup {
    html! {
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
    }
}
