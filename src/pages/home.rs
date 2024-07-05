use maud::{Markup, html};
use crate::components::layout::layout;

pub fn render() -> Markup {
    layout(
        "Free Methodist Bible Quizzing",
        html! {
            .hero.full-width {
                h1 { "Free Methodist Bible Quizzing" }
            }

            .section.dark {
                p {
                    "Looking to start Bible Quizzing in your church?"

                    a.button href="/get-started" {
                        "Get Started"
                    }
                }
            }
        },
    )
}
