use crate::{domain::quotes::get_season_by_id, web::components::layout::layout};

use maud::{html, Markup};

pub fn render(year: u64) -> Markup {
    let season = get_season_by_id(year);
    let docs_folder = "/static/resources/docs/";

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
                    a.button href={(docs_folder)(season.pdf)} {"pdf"}
                    "  "
                    a.button href={(docs_folder)(season.text)} {"txt"}
                }
            }
            @else {
                h2 { "Season not found!" }
            }
        },
    )
}
