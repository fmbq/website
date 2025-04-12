use crate::web::components::layout::layout;

use maud::{html, Markup};

pub fn render(year: u64) -> Markup {
    //let image_folder = "/static/resources/photos/finals/";
    layout(
        format!("{} Finals",year),
        html! {
            h1 { (year)(" Finals") }
            p {
                a href={"/seasons/"(year)"/finals/registration"} {"Registration Information"}
                br;
                a href={"/seasons/"(year)"/finals/news"} {"Latest News"}
                br;
                a href={"/seasons/"(year)"/finals/schedule"} {"Schedule"}
                br;
                a href={"/seasons/"(year)"/finals/tournament"} {"Tournament - Live during finals week"}
                br;
                a href={"/seasons/"(year)"/finals/awards"} {"Finals Results"}
                br;
            }
            p{}
            p{}
        },
    )
}
