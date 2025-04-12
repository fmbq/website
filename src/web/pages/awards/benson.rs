use crate::web::components::layout::layout;

use maud::{html, Markup};

pub fn render() -> Markup {
    let image_folder = "/static/resources/photos/awards/";
    layout(
        "Benson",
        html! {
            h1 { "Benson" }
            img src={(image_folder)("benson-2015.jpg")} height="300px" {""}
            br;
            p{}
            p{}
        },
    )
}
