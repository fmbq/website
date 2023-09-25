use maud::{Markup, html};
use crate::components::layout::layout;

pub fn render() -> Markup {
    layout(
        "Free Methodist Bible Quizzing",
        html! {
            .hero {
                .container {
                    h1 { "Free Methodist Bible Quizzing" }
                }
            }
        },
    )
}
