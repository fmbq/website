use super::{pages, sse};
use ::time::{format_description::well_known::Rfc2822, OffsetDateTime};
use maud::Markup;
use poem::{
    handler,
    web::{sse::SSE, Path, Redirect},
    IntoResponse,
};

pub mod admin;
pub mod article_images;
pub mod articles;
pub mod css;
pub mod errors;
pub mod photos;
pub mod rules;
pub mod signup;

#[handler]
pub fn home() -> Markup {
    pages::home::render()
}

#[handler]
pub fn about() -> Markup {
    pages::about::render()
}

#[handler]
pub fn schedule() -> Markup {
    pages::schedule::render()
}

#[handler]
pub fn resources() -> Markup {
    pages::resources::render()
}

#[handler]
pub fn awards() -> Markup {
    pages::awards::render()
}

#[handler]
pub fn support_us() -> Markup {
    pages::support_us::render()
}

#[handler]
pub fn contacts() -> Markup {
    pages::contacts::render()
}

#[handler]
pub fn playground() -> Markup {
    pages::playground::render()
}

#[handler]
pub fn quotes_root() -> impl IntoResponse {
    Redirect::see_other(format!("/quotes/{}", OffsetDateTime::now_utc().year()))
}

#[handler]
pub fn quotes_for_year(Path(year): Path<u64>) -> Markup {
    pages::quotes::render(year)
}

#[handler]
pub fn hall_of_fame() -> Markup {
    pages::hall_of_fame::render()
}

#[handler]
pub fn markell() -> Markup {
    pages::markell::render()
}

#[handler]
pub fn material() -> Markup {
    pages::material::render()
}

#[handler]
pub fn events() -> SSE {
    sse::subscribe()
}

#[handler]
pub fn time() -> impl IntoResponse {
    OffsetDateTime::now_utc().format(&Rfc2822).unwrap()
}
