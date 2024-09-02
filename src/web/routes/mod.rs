use super::pages;
use maud::Markup;
use ::time::{format_description::well_known::Rfc2822, OffsetDateTime};
use color_eyre::eyre::{bail, Result};
use poem::{
    endpoint::{EmbeddedFilesEndpoint, StaticFilesEndpoint},
    get, handler,
    web::{
        sse::{Event, SSE},
        Html,
    },
    EndpointExt, IntoResponse, Route,
};

pub mod admin;
pub mod article_images;
pub mod articles;
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
pub fn contacts() -> Html<Markup> {
    Html(pages::contacts::render())
}

#[handler]
pub fn playground() -> Html<Markup> {
    Html(pages::playground::render())
}

#[handler]
pub fn resources() -> Html<Markup> {
    Html(pages::resources::render())
}

#[handler]
pub fn quotes() -> Html<Markup> {
    Html(pages::quotes::render())
}

#[handler]
pub fn events() -> SSE {
    crate::sse::subscribe()
}

#[handler]
pub fn css() -> impl IntoResponse {
    grass::include!("src/web/scss/index.scss").with_content_type("text/css")
}

#[handler]
pub fn admin_css() -> impl IntoResponse {
    grass::include!("src/web/scss/admin.scss").with_content_type("text/css")
}

#[handler]
pub fn time() -> impl IntoResponse {
    OffsetDateTime::now_utc().format(&Rfc2822).unwrap()
}
