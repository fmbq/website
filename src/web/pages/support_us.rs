use crate::web::components::layout::layout;
use maud::{html, Markup};

pub fn render() -> Markup {
    layout(
        "Support Us",
        html! {
            h1 { "Support Us" }

            h2 { "Prayer" }

            p {
                "Support us through prayer."
            }

            h2 { "Financially" }

            p {
                "Support us financially."
            }
        },
    )
}
