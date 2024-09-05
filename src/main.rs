#![allow(dead_code)]

use color_eyre::eyre::{bail, Result};
use poem::{
    endpoint::{EmbeddedFilesEndpoint, StaticFilesEndpoint},
    get,
    web::sse::Event,
    EndpointExt, Route,
};
use rust_embed::Embed;
use std::env;
use web::routes;

mod config;
mod db;
mod domain;
mod services;
mod session;
mod util;
mod web;

#[derive(Embed)]
#[folder = "js"]
struct JsDirectory;

#[tokio::main]
async fn main() -> Result<()> {
    if dotenvy::dotenv().is_err() {
        eprintln!("no .env file found, did you need to create one?");
    }

    color_eyre::install()?;

    tracing_subscriber::fmt::init();

    let Some(project_dirs) =
        directories::ProjectDirs::from("org.fmquizzing", "FM Quizzing", "FM Quizzing Website")
    else {
        bail!("failed to locate project directories");
    };

    tracing::info!("initializing database");
    db::init().await?;

    let app = Route::new()
        .at("/", get(routes::home))
        .at("/about", get(routes::about))
        .at("/contacts", get(routes::contacts))
        .at("/resources", get(routes::resources))
        .at("/quotes", get(routes::quotes))
        .at("/rules", get(routes::rules::get_html))
        .at("/rules/rules.pdf", get(routes::rules::get_pdf))
        .at("/playground", get(routes::playground))
        .at("/time", get(routes::time))
        .at("/events", get(routes::events))
        .nest("/admin", routes::admin::routes())
        .nest("/styles", routes::css::routes())
        .nest("/js", EmbeddedFilesEndpoint::<JsDirectory>::new())
        .at(
            "/static/resources/photos/:image",
            get(routes::photos::get_photo),
        )
        .nest("/static", StaticFilesEndpoint::new("wwwroot/static"))
        .with(web::middleware::headers::security_headers())
        .data(project_dirs)
        .data(db::create_connection_pool()?)
        .data(services::email::Mailer::new()?);

    let app = session::configure_session(app).await?;

    tokio::task::spawn(async {
        loop {
            web::sse::publish(Event::message("updated").event_type("time-update"));
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });

    let addr = env::var("LISTEN_ADDR").unwrap_or_else(|_| String::from("127.0.0.1:80"));

    tracing::info!("listening on http://{}", addr);

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
