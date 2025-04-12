use maud::Markup;
use poem::{get, handler, web::Path, IntoEndpoint, Route};

use crate::web::pages;


pub fn routes() -> impl IntoEndpoint {
    Route::new()
        .at("/", get(seasons))
        .at("/:year/quotes", get(season_quotes_for_year))
        .at("/:year/finals", get(season_finals_for_year))
        .at("/:year/finals/awards", get(season_finals_awards_for_year))
        .at("/:year/finals/registration", get(season_finals_registration_for_year))
        .at("/:year/finals/news", get(season_finals_news_for_year))
        .at("/:year/finals/schedule", get(season_finals_schedule_for_year))
        .at("/:year/finals/tournaments", get(season_finals_tournaments_for_year))
}

#[handler]
pub fn seasons() -> Markup {
    pages::seasons::render()
}

#[handler]
pub fn season_quotes_for_year(Path(year): Path<u64>) -> Markup {
    pages::seasons::quotes::render(year)
}

#[handler]
pub fn season_finals_for_year(Path(year): Path<u64>) -> Markup {
    pages::seasons::finals::render(year)
}

#[handler]
pub fn season_finals_awards_for_year(Path(year): Path<u64>) -> Markup {
    pages::seasons::finals_awards::render(year)
}

#[handler]
pub fn season_finals_registration_for_year(Path(year): Path<u64>) -> Markup {
    pages::seasons::finals_registration::render(year)
}

#[handler]
pub fn season_finals_news_for_year(Path(year): Path<u64>) -> Markup {
    pages::seasons::finals_news::render(year)
}

#[handler]
pub fn season_finals_schedule_for_year(Path(year): Path<u64>) -> Markup {
    pages::seasons::finals_schedule::render(year)
}

#[handler]
pub fn season_finals_tournaments_for_year(Path(year): Path<u64>) -> Markup {
    pages::seasons::finals_tournament::render(year)
}

