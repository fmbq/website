use crate::{
    db::Pool,
    domain::articles::list_articles,
    web::{
        middleware::auth::LoginCheckMiddleware,
        pages::admin::{article_management, index},
    },
};
use maud::Markup;
use poem::{
    get, handler,
    web::{Data, Html},
    EndpointExt, IntoEndpoint, Route,
};

pub mod auth;

pub fn routes() -> impl IntoEndpoint {
    Route::new()
        .at("/", get(get_index))
        .at("/login", get(auth::login::get).post(auth::login::submit))
        .at("/logout", get(auth::logout::get))
        .at(
            "/request-password-reset",
            get(auth::reset_password::request_form::get)
                .post(auth::reset_password::request_form::submit),
        )
        .at(
            "/reset-password",
            get(auth::reset_password::reset_form::get)
                .post(auth::reset_password::reset_form::submit),
        )
        .at("/articles", get(get_article_management))
        .with(LoginCheckMiddleware)
}

#[handler]
pub async fn get_index() -> Html<Markup> {
    Html(index())
}

#[handler]
pub async fn get_article_management(Data(db): Data<&Pool>) -> Html<Markup> {
    let mut conn = db.acquire().await.unwrap();
    let articles = list_articles(&mut conn).await;

    Html(article_management(&articles))
}
