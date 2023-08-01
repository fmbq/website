use poem::{endpoint::StaticFilesEndpoint, get, web::Html, Route, IntoResponse};
use std::env;

mod pages;

#[poem::handler]
fn home() -> Html<String> {
    Html(pages::home::render().into_string())
}

#[poem::handler]
fn about() -> Html<String> {
    Html(pages::about().into_string())
}

#[poem::handler]
fn css() -> impl IntoResponse {
    grass::include!("src/scss/index.scss").with_content_type("text/css")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    let app = Route::new()
        .at("/", get(home))
        .at("/about", get(about))
        .at("/styles/site.css", get(css))
        .nest("/resources", StaticFilesEndpoint::new("wwwroot/resources"))
        .nest("/js", StaticFilesEndpoint::new("wwwroot/js"));

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
