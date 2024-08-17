//! A hidden page for testing out features.

use crate::components::layout::layout;
use maud::{html, Markup};

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
