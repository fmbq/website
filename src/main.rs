use ::time::{format_description::well_known::Rfc2822, OffsetDateTime};
use poem::{
    endpoint::StaticFilesEndpoint,
    get,
    web::{
        sse::{Event, SSE},
        Html,
    },
    IntoResponse, Route,
};
use std::env;

mod components;
mod pages;
mod sse;

#[poem::handler]
fn home() -> Html<String> {
    Html(pages::home::render().into_string())
}

#[poem::handler]
fn about() -> Html<String> {
    Html(pages::about().into_string())
}

#[poem::handler]
fn playground() -> Html<String> {
    Html(pages::playground::render().into_string())
}

#[poem::handler]
fn events() -> SSE {
    sse::subscribe()
}

#[poem::handler]
fn css() -> impl IntoResponse {
    grass::include!("src/scss/index.scss").with_content_type("text/css")
}

#[poem::handler]
fn time() -> impl IntoResponse {
    OffsetDateTime::now_utc().format(&Rfc2822).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    let app = Route::new()
        .at("/", get(home))
        .at("/about", get(about))
        .at("/playground", get(playground))
        .at("/time", get(time))
        .at("/events", get(events))
        .at("/styles/site.css", get(css))
        .nest("/static", StaticFilesEndpoint::new("wwwroot/static"));

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
        .await
}
