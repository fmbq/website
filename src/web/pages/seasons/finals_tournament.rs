use crate::web::components::layout::layout;

use maud::{html, Markup};

pub fn render(year: u64) -> Markup {
    // let image_folder = "/static/resources/photos/finals/";
    layout(
        format!("{} Finals Tournament",year),
        html! {
            h1 { (year)(" Finals Tournament") }
            p {
                "Include ways to watch the tournament."
                br;
                "Show current standings, round results, etc."
                br;
            }
            p{}
            p{}
        },
    )
}
