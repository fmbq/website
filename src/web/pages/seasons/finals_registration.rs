use crate::web::components::layout::layout;

use maud::{html, Markup};

pub fn render(year: u64) -> Markup {
    // let image_folder = "/static/resources/photos/finals/";
    layout(
        format!("{} Finals Registration",year),
        html! {
            h1 { (year)(" Finals Registration") }
            p {
                "Information about how to register for finals. Should include links to forms or spreadsheets."
                br;
                "Could include registration status for each team."
                br;
                "Include due dates, costs, etc."
                br;
            }
            p{}
            p{}
        },
    )
}
