use crate::web::components::layout::layout;

use maud::{html, Markup};

pub fn render() -> Markup {
    let image_folder = "/static/resources/photos/awards/";
    layout(
        "Alpha Omega",
        html! {
            h1 { "Alpha Omega" }
            img src={(image_folder)("alpha-omega-2015.webp")} height="300px" {""}
            br;
            p{}
            p{}
        },
    )
}
