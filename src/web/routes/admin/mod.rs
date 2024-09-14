use crate::{
    db::Pool,
    domain::articles::list_articles,
    web::{
        login_context::LoginContext,
        middleware::auth::LoginCheckMiddleware,
        pages::admin::{AdminModule, ArticleManagement, Index},
    },
};
use maud::Markup;
use poem::{
    get, handler, post,
    web::{Data, Html},
    EndpointExt, IntoEndpoint, Route,
};

mod account_settings;
mod auth;

pub fn routes() -> impl IntoEndpoint {
    Route::new()
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
        .nest_no_strip(
            "/",
            Route::new()
                .at("/", get(get_index))
                .at("/account", get(account_settings::get))
                .at(
                    "/account/change-password",
                    post(account_settings::submit_change_password),
                )
                .at("/articles", get(get_article_management))
                .with(LoginCheckMiddleware),
        )
}

#[handler]
async fn get_index(login_context: LoginContext) -> Html<Markup> {
    Html(Index.render(&login_context))
}

#[handler]
async fn get_article_management(
    login_context: LoginContext,
    Data(db): Data<&Pool>,
) -> Html<Markup> {
    let mut conn = db.acquire().await.unwrap();
    let articles = list_articles(&mut conn).await;

    Html(
        ArticleManagement {
            articles: &articles,
        }
        .render(&login_context),
    )
}
