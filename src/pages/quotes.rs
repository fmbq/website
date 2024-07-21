use crate::{components::layout::layout, domain::quotes::get_season_by_id};

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
                //TODO: should use a flex-box with two columns
                @for month in &season.month {
                    h3 { "Month " (month.id) ": " (month.material) }
                    ul {
                        @for quote in &month.quote {
                            li { (quote.reference) }
                        }
                    }
                    i { (month.quote.len()) " quotes / " (month.verses) " verses"}
                }

                p{
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
