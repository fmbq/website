use crate::components::layout::layout;
use maud::{html, Markup};

pub fn render() -> Markup {
    layout(
        "Quote List",
        html! {
            h1 { "Quote List" }
            h2 { "Romans & James"}
            h4 { "Total: 541 verses, 48 Quotes" }
            p{
                //quick xml
                a.button href="/static/resources/docs/2024 Quote List Romans and James.pdf" {"pdf"}
                a.button href="/static/resources/docs/2024 Quote List Romans and James.txt" {"txt"}
            }          
        },
    )
}
