use crate::web::components::layout::layout;

use maud::{html, Markup};

pub fn render(year: u64) -> Markup {
    // let image_folder = "/static/resources/photos/finals/";
    layout(
        format!("{} Finals Awards",year),
        html! {
            h1 { (year)(" Finals Awards") }
            p {
                "STV - 1st - winner here"
                br;
                "STV - 2nd - winner here"
                br;
                "STV - 3rd - winner here"
                br;
                "STV - 4th - winner here"
                br;
                "... and so on"
                br;
            }
            p{}
            p{}
        },
    )
}
