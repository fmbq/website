use crate::{
    db::Pool,
    domain::{articles::list_articles, user::get_profile},
    web::{
        middleware::auth::LoginCheckMiddleware,
        pages::admin::{article_management, index, user_profile}, session::LoginSession,
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
                .at("/profile", get(get_user_profile))
                .at("/articles", get(get_article_management))
                .with(LoginCheckMiddleware),
        )
}

#[handler]
async fn get_index() -> Html<Markup> {
    Html(index())
}

#[handler]
async fn get_article_management(Data(db): Data<&Pool>) -> Html<Markup> {
    let mut conn = db.acquire().await.unwrap();
    let articles = list_articles(&mut conn).await;

    Html(article_management(&articles))
}

#[handler]
async fn get_user_profile(
    login_session: LoginSession,
    Data(db): Data<&Pool>,
) -> Html<Markup> {
    let mut conn = db.acquire().await.unwrap();

    let user = get_profile(&mut conn, &login_session.user_id).await.unwrap().unwrap();

    Html(user_profile(login_session, &user))
}
