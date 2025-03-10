use crate::web::components::layout::layout;
use maud::{html, Markup};

pub fn render() -> Markup {
    layout(
        "Hall of Fame Members",
        html! {
            h1 { "Hall of Fame Members" }
            p {
                img src="/static/resources/photos/alpha-omega.jpg" height="400px" {""}
            }

        },
    )
}
