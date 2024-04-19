use maud::{html, Markup};

use crate::db::articles::ArticleSummary;

pub fn article_list(articles: &[ArticleSummary]) -> Markup {
    html! {
        table {
            thead {
                tr {
                    th { "Title" }
                    th { "Actions" }
                }
            }

            tbody {
                @for article in articles {
                    tr {
                        td { (article.title.as_str()) }
                        td {
                            a href=(format!("/admin/articles/{}", article.id)) { "Edit" }
                            a href=(format!("/admin/articles/{}", article.id)) { "Delete" }
                        }
                    }
                }
            }
        }

        button {
            "Create new"
        }
    }
}
