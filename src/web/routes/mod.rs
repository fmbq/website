use super::{pages, sse};
use ::time::{format_description::well_known::Rfc2822, OffsetDateTime};
use maud::Markup;
use poem::{
    handler,
    web::{sse::SSE, Html},
    IntoResponse,
};

pub mod admin;
pub mod article_images;
pub mod articles;
pub mod css;
pub mod photos;
pub mod rules;
pub mod signup;

#[handler]
pub fn home() -> Html<Markup> {
    Html(pages::home::render())
}

#[handler]
pub fn about() -> Html<Markup> {
    Html(pages::about::render())
}

#[handler]
pub fn schedule() -> Html<Markup> {
    Html(pages::schedule::render())
}

#[handler]
pub fn resources() -> Html<Markup> {
    Html(pages::resources::render())
}

#[handler]
pub fn awards() -> Html<Markup> {
    Html(pages::awards::render())
}

#[handler]
pub fn support_us() -> Html<Markup> {
    Html(pages::support_us::render())
}

#[handler]
pub fn contacts() -> Html<Markup> {
    Html(pages::contacts::render())
}

#[handler]
pub fn playground() -> Html<Markup> {
    Html(pages::playground::render())
}

#[handler]
pub fn quotes() -> Html<Markup> {
    Html(pages::quotes::render())
}

#[handler]
pub fn hall_of_fame() -> Html<Markup> {
    Html(pages::hall_of_fame::render())
}

#[handler]
pub fn events() -> SSE {
    sse::subscribe()
}

#[handler]
pub fn time() -> impl IntoResponse {
    OffsetDateTime::now_utc().format(&Rfc2822).unwrap()
}
