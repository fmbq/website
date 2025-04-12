use crate::{
    domain::finals::get_finals,
    web::components::layout::layout,
};

use maud::{html, Markup};

pub fn render() -> Markup {
    let finals = get_finals();
    layout(
        "Finals",
        html! {
            h1 { "Past Finals Locations" }

            @for item in &finals.finals {
                div {
                    (item.year) " " (item.place) " " (item.city) " " (item.state)
                    br;
                }
            }
            p{}
            p{}
        },
    )
}
