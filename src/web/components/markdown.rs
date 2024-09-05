use crate::services::markdown;
use maud::{html, Markup, PreEscaped};

pub fn markdown(text: &str) -> Markup {
    html! {
        (PreEscaped(markdown::render_html(text)))
    }
}
