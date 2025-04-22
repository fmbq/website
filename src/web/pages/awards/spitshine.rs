use crate::web::components::layout::layout;

use maud::{html, Markup};

pub fn render() -> Markup {
    let image_folder = "/static/resources/photos/awards/";
    layout(
        "Spitshine",
        html! {
            h1 { "Spitshine" }
            img src={(image_folder)("spitshine-2011.webp")} height="300px" {""}
            br;
            p{}
            p{}
        },
    )
}
