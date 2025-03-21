use crate::web::components::layout::layout;
use maud::{html, Markup};

pub fn not_found() -> Markup {
    layout(
        "Not Found",
        html! {
            h1 { "Not Found" }

            p{ "Sorry, I couldn't find anything for this page." }
        },
    )
}
