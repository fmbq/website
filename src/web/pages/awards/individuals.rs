use crate::web::components::layout::layout;

use maud::{html, Markup};

pub fn render() -> Markup {
    let image_folder = "/static/resources/photos/awards/";
    layout(
        "Individuals",
        html! {
            h1 { "Individuals" }
            img src={(image_folder)("individuals-awards.jpg")} height="300px" {""}
            br;
            p{}
            p{}
        },
    )
}
