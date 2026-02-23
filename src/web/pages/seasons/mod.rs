use crate::{
    domain::{material::get_material, seasons::get_current_season_year},
    web::components::layout::layout,
};
use itertools::Itertools;
use maud::{html, Markup};

pub mod finals;
pub mod finals_awards;
pub mod finals_news;
pub mod finals_registration;
pub mod finals_schedule;
pub mod finals_tournament;
pub mod quotes;

pub fn render() -> Markup {
    let material = get_material();
    let current_season_year = get_current_season_year();
    let current_season = material.for_season_year(current_season_year);
    let upcoming_seasons = material
        .materials
        .iter()
        .filter(|m| m.year > current_season_year)
        // Show soonest upcoming seasons first.
        .sorted_by(|a, b| a.year.cmp(&b.year));
    let past_seasons = material
        .materials
        .iter()
        .filter(|m| m.year < current_season_year);

    layout(
        "Seasons",
        html! {
            h1 { "Seasons" }

            @if let Some(current_season) = current_season {
                h2 { "Current Season" }
                h3 { (current_season.title()) }
                h3 { (current_season.books) }

                p {
                    a.button href={"/seasons/"(current_season.year)} {"View Season Details"}
                }
            }

            h2 { "Upcoming Seasons" }
            ul {
                @for item in upcoming_seasons {
                    li {
                        a href={"/seasons/"(item.year)} {
                            (item.title())
                        }
                    }
                }
            }

            h2 { "Past Seasons" }
            ul {
                @for item in past_seasons {
                    li {
                        a href={"/seasons/"(item.year)} {
                            (item.title())
                        }
                    }
                }
            }
        },
    )
}

pub fn render_year(year: u64) -> Option<Markup> {
    let this_material = get_material().for_season_year(year as u16)?;

    Some(layout(
        this_material.title(),
        html! {
            h1 { (this_material.title()) }
            h2 { (this_material.books) }

            p {
                a.button href={"/seasons/"(year)"/quotes"} {"Quotes"}
                br;
                a.button href={"/seasons/"(year)"/finals"} {"Finals"}
            }
        },
    ))
}
