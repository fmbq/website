use crate::components::layout::layout;
use crate::domain::quotes::get_season_by_id;

use maud::{html, Markup};

pub fn render() -> Markup {
    let season = get_season_by_id(2024);
    layout(
        "Quote List",
        html! {
            h1 { "Quote List" }
            @if let Some(season) = season {
                h2 { (season.books) }
                h4 { "Total: "(season.verses)" verses, "(season.get_quote_count())" Quotes" }
                p{
                    //quick xml

                    a.button href="/static/resources/docs/2024 Quote List Romans and James.pdf" {"pdf"}
                    a.button href="/static/resources/docs/2024 Quote List Romans and James.txt" {"txt"}
                }          
            }
            @else {
                h2 { "Season not found!" }
            }
        },
    )
}
