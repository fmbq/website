use maud::{html, Markup, PreEscaped};

pub fn markdown(text: &str) -> Markup {
    html! {
        (PreEscaped(crate::markdown::render_html(text)))
    }
}
