use maud::{Markup, html};
use super::layout;

pub fn render() -> Markup {
    layout(
        "Free Methodist Bible Quizzing",
        html! {
            h1 { "Home" }
        },
    )
}
