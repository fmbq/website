use crate::{
    domain::markell::{get_markell, Markell },
    web::components::layout::layout,
};

use maud::{html, Markup};

pub fn render() -> Markup {
    let markell = get_markell();
    let markell_image_folder = "./static/resources/photos/markell/";
    layout(
        "Markell",
        html! {
            h1 { "Dave Markell Excellence in Attitude Award" }
            img src={(markell_image_folder)(markell.image)} height="300px" {""}
            br;
            h2 {(markell.bio)}

            @for award in &markell.winners {
                div {
                    (award.year) " " (award.name) " " (award.location)
                    br;
                }
            }
            p{}
            p{}
        },
    )
}
