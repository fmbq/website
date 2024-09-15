use maud::{html, Markup};

use crate::db::users::User;

pub fn account_menu(user: &User) -> Markup {
    html! {
        #account-menu _="on click elsewhere remove .active" {
            .menu-button _="on click toggle .active on #account-menu" {
                img src="/static/resources/images/user.svg";
                span {
                    (user.display_name())
                }
            }
            menu {
                a href="/admin/account" { "Account settings" }
                a href="/admin/logout" { "Log Out" }
            }
        }
    }
}
