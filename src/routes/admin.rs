use crate::{
    components::{
        admin::{article_list::article_list, sidebar::sidebar},
        admin_layout::admin_layout,
    },
    db::Pool,
    domain::articles::list_articles,
};
use maud::{html, Markup};
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Form, Html, Path},
    IntoResponse, Response,
};

#[handler]
pub async fn get_article_management(Data(db): Data<&Pool>) -> Html<Markup> {
    let mut conn = db.acquire().await.unwrap();
    let articles = list_articles(&mut conn).await;

    Html(admin_layout(
        "Articles",
        html! {
            (sidebar())

            (article_list(&articles))
        },
    ))
}
