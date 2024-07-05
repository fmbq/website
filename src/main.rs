#![allow(dead_code)]

use maud::Markup;
use rust_embed::Embed;
use ::time::{format_description::well_known::Rfc2822, OffsetDateTime};
use poem::{
    endpoint::{EmbeddedFilesEndpoint, StaticFilesEndpoint},
    get, handler,
    web::{
        sse::{Event, SSE},
        Html,
    },
    EndpointExt, IntoResponse, Route,
};
use std::env;

mod components;
mod db;
mod domain;
mod markdown;
mod pages;
mod routes;
mod session;
mod sse;
mod url;

#[handler]
fn home() -> Html<Markup> {
    Html(pages::home::render())
}

#[handler]
fn about() -> Html<Markup> {
    Html(pages::about())
}

#[handler]
fn playground() -> Html<Markup> {
    Html(pages::playground::render())
}

#[handler]
fn events() -> SSE {
    sse::subscribe()
}

#[handler]
fn css() -> impl IntoResponse {
    grass::include!("src/scss/index.scss").with_content_type("text/css")
}

#[handler]
fn admin_css() -> impl IntoResponse {
    grass::include!("src/scss/admin.scss").with_content_type("text/css")
}

#[handler]
fn time() -> impl IntoResponse {
    OffsetDateTime::now_utc().format(&Rfc2822).unwrap()
}

#[derive(Embed)]
#[folder = "js"]
struct JsDirectory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if dotenv::dotenv().is_err() {
        eprintln!("no .env file found, did you need to create one?");
    }

    env_logger::init();

    log::info!("initializing database");
    db::init().await?;

    let app = Route::new()
        .at("/", get(home))
        .at("/about", get(about))
        .at("/playground", get(playground))
        .at("/time", get(time))
        .at("/events", get(events))
        .at(
            "/admin/login",
            get(routes::login::get).post(routes::login::submit),
        )
        .at(
            "/admin/articles",
            get(routes::admin::get_article_management),
        )
        .at("/styles/site.css", get(css))
        .at("/styles/admin.css", get(admin_css))
        .nest("/js", EmbeddedFilesEndpoint::<JsDirectory>::new())
        .nest("/static", StaticFilesEndpoint::new("wwwroot/static"))
        .data(db::create_connection_pool()?);

    let app = session::configure_session(app).await?;

    tokio::task::spawn(async {
        loop {
            sse::publish(Event::message("updated").event_type("time-update"));
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });

    let addr = env::var("LISTEN_ADDR").unwrap_or_else(|_| String::from("127.0.0.1:80"));

    log::info!("listening on http://{}", addr);

    poem::Server::new(poem::listener::TcpListener::bind(addr))
        .run_with_graceful_shutdown(
            app,
            async {
                tokio::signal::ctrl_c().await.ok();
            },
            None,
        )
        .await?;

    Ok(())
}
