//! A hidden page for testing out features.

use maud::{Markup, html};
use crate::components::layout::layout;

pub fn render() -> Markup {
    layout(
        "Free Methodist Bible Quizzing",
        html! {
            h1 { "Playground" }

            div hx-ext="sse" sse-connect="/events" {
                div hx-get="/time" hx-trigger="sse:time-update" {
                    "[time]"
                }
            }

        },
    )
}
