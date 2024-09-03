use crate::{
    db::articles::ArticleSummary,
    web::components::{
        admin::{article_list::article_list, sidebar::sidebar},
        admin_layout::admin_layout,
    },
};
use maud::{html, Markup};

pub fn index() -> Markup {
    admin_layout(
        "Index",
        html! {
            (sidebar())

            p { "Index" }
        },
    )
}

pub fn article_management(articles: &[ArticleSummary]) -> Markup {
    admin_layout(
        "Articles",
        html! {
            (sidebar())

            (article_list(articles))
        },
    )
}
