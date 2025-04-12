use crate::web::components::layout::layout;

use maud::{html, Markup};

pub fn render(year: u64) -> Markup {
    // let image_folder = "/static/resources/photos/finals/";
    layout(
        format!("{} Finals Schedule",year),
        html! {
            h1 { (year)(" Finals Schedule") }
            p {
                "Daily schedule in HTML and PDF formats."
                br;
                "Team competition schedule."
                br;
            }
            p{}
            p{}
        },
    )
}
