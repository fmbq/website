use crate::{
    db::articles::ArticleSummary,
    web::{
        components::{
            admin::{account_menu::account_menu, article_list::article_list, change_password, sidebar::sidebar, user_info},
            copyright::copyright,
            scripts::scripts,
        },
        login_context::LoginContext,
    },
};
use maud::{html, Markup, DOCTYPE};

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

                aside.sidebar {
                    (sidebar())
                }

                main role="main" {
                    h2 { (self.title()) }

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
            p { "User Email: " (&login_context.user.email) }

            (user_info::form(&login_context.user))

            (change_password::form())
        }
    }
}
