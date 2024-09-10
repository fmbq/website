use crate::{
    db::{articles::ArticleSummary, users::User},
    web::{components::{
        admin::{article_list::article_list, sidebar::sidebar},
        admin_layout::admin_layout,
    }, session::LoginSession},
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

pub fn user_profile(
    login_session: LoginSession,
    user: &User,
) -> Markup {
    admin_layout(
        "User Profile",
        html! {
            (sidebar())

            p { "User ID: " (login_session.user_id) }
            p { "User Email: " (user.email) }
        },
    )
}
