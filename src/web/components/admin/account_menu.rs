use maud::{html, Markup};

pub fn account_menu() -> Markup {
    html! {
        #account-menu _="on click elsewhere remove .active" {
            .menu-button _="on click toggle .active on #account-menu" {
                img src="/static/resources/images/user.svg";
                span {
                    "My Account"
                }
            }
            menu {
                a href="/admin/profile" { "Profile" }
                a href="/admin/logout" { "Log Out" }
            }
        }
    }
}
