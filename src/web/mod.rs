//! Everything related to web endpoints and HTML rendering is contained within
//! this module.

use poem::{
    endpoint::{EmbeddedFilesEndpoint, StaticFileEndpoint, StaticFilesEndpoint},
    get, EndpointExt, IntoEndpoint, Route,
};
use rust_embed::Embed;

pub mod components;
pub mod sse;

mod login_context;
mod middleware;
mod pages;
mod routes;

/// Root endpoint for the whole website. All routes are wired up here.
pub fn root() -> impl IntoEndpoint {
    #[derive(Embed)]
    #[folder = "js"]
    struct JsDirectory;

    Route::new()
        .at("/", get(routes::home))
        .at("/about", get(routes::about))
        .at("/resources", get(routes::resources))
        .at("/support-us", get(routes::support_us))
        .at("/contacts", get(routes::contacts))
        .at("/quotes", get(routes::quotes_root))
        .at("/quotes/:year", get(routes::quotes_for_year))
        .at("/rules", get(routes::rules::get_html))
        .at("/rules/rules.pdf", get(routes::rules::get_pdf))
        .at("/material", get(routes::material))
        .at("/playground", get(routes::playground))
        .at("/time", get(routes::time))
        .at("/events", get(routes::events))
        .nest("/awards", routes::awards::routes())
        .nest("/seasons", routes::seasons::routes())
        .nest("/admin", routes::admin::routes())
        .nest("/styles", routes::css::routes())
        .nest("/js", EmbeddedFilesEndpoint::<JsDirectory>::new())
        .at(
            "/static/resources/photos/:image",
            get(routes::photos::get_photo),
        )
        .nest("/static", StaticFilesEndpoint::new("wwwroot/static"))
        .at(
            "/favicon.ico",
            StaticFileEndpoint::new("wwwroot/favicon.ico"),
        )
        .at(
            "/icon.svg",
            StaticFileEndpoint::new("wwwroot/icon.svg"),
        )
        .at(
            "/apple-touch-icon.png",
            StaticFileEndpoint::new("wwwroot/apple-touch-icon.png"),
        )
        .at("/robots.txt", get(routes::robots))
        .catch_error(routes::errors::not_found)
        .with(middleware::boost::BoostMiddleware)
        .with(middleware::headers::security_headers())
}
