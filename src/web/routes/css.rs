use poem::{get, handler, EndpointExt, IntoEndpoint, IntoResponse, Route};

pub fn routes() -> impl IntoEndpoint {
    Route::new()
        .at("site.css", get(index))
        .at("login.css", get(login))
        .at("admin.css", get(admin))
        .map(|r| async move { r.with_content_type("text/css") })
}

#[handler]
fn index() -> impl IntoResponse {
    grass::include!("src/web/scss/index.scss")
}

#[handler]
fn login() -> impl IntoResponse {
    grass::include!("src/web/scss/login.scss")
}

#[handler]
fn admin() -> impl IntoResponse {
    grass::include!("src/web/scss/admin.scss")
}
