use crate::web::components::layout::layout;

use maud::{html, Markup};

pub fn render(year: u64) -> Markup {
    // let image_folder = "/static/resources/photos/finals/";
    layout(
        format!("{} Finals News",year),
        html! {
            h1 { (year)(" Finals News") }
            p {
                "Links to posts about finals."
                br;
            }
            p{}
            p{}
        },
    )
}
