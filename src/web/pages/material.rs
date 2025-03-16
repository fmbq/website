use crate::{
    domain::material::get_material,
    web::components::layout::layout,
};

use maud::{html, Markup};

pub fn render() -> Markup {
    let material = get_material();
    layout(
        "Material",
        html! {
            h1 { "All Material Studied" }

            @for item in &material.materials {
                div {
                    (item.year) " " (item.books) " " (item.translation)
                    br;
                }
            }
            p{}
            p{}
        },
    )
}
