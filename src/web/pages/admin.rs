use crate::{
    db::{articles::ArticleSummary, users::User},
    web::{components::{
        admin::{article_list::article_list, sidebar::sidebar, account_menu::account_menu},
        copyright::copyright, scripts::scripts,
    }, login_context::LoginContext},
};
use maud::{Markup, html, DOCTYPE};

pub trait AdminModule {
    fn title(&self) -> &str;

    fn body(&self, login_context: &LoginContext) -> Markup;

    fn render(&self, login_context: &LoginContext) -> Markup {
        html! {
            (DOCTYPE)
            head lang="en" {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";

                title { (self.title()) }

                link rel="stylesheet" href="/styles/admin.css";

                (scripts())
            }
            body {
                header {
                    a href="/admin" { "Home" }

                    .flex-end {
                        (account_menu(&login_context.user))
                    }
                }

                main role="main" {
                    (sidebar())

                    (self.body(login_context))
                }

                footer {
                    div class="container" {
                        (copyright())
                    }
                }
            }
        }
    }
}

pub struct Index;

impl AdminModule for Index {
    fn title(&self) -> &str {
        "Index"
    }

    fn body(&self, _login_context: &LoginContext) -> Markup {
        html! {
            p { "Index" }
        }
    }
}

pub struct ArticleManagement<'a> {
    pub articles: &'a [ArticleSummary],
}

impl AdminModule for ArticleManagement<'_> {
    fn title(&self) -> &str {
        "Articles"
    }

    fn body(&self, _login_context: &LoginContext) -> Markup {
        html! {
            (article_list(self.articles))
        }
    }
}

pub struct AccountSettings;

impl AdminModule for AccountSettings {
    fn title(&self) -> &str {
        "Account Settings"
    }

    fn body(&self, login_context: &LoginContext) -> Markup {
        html! {
            p { "User ID: " (&login_context.user_id) }
            p { "User Email: " (&login_context.user.email) }
        }
    }
}
