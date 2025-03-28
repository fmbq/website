#![allow(dead_code)]

use color_eyre::eyre::{bail, Result};
use poem::{web::sse::Event, EndpointExt};
use std::env;

mod config;
mod db;
mod domain;
mod services;
mod session;
mod util;
mod web;

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

    let app = web::root()
        .data(config::DeploymentEnvironment::from_env())
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
