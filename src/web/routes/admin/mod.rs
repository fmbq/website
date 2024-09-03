use crate::{
    db::Pool,
    domain::articles::list_articles,
    web::components::{
        admin::{article_list::article_list, sidebar::sidebar},
        admin_layout::admin_layout,
    },
};
use maud::{html, Markup};
use poem::{
    handler,
    web::{Data, Html},
};

pub mod auth;

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
