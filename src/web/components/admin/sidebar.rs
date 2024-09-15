use maud::{html, Markup};

pub fn sidebar() -> Markup {
    html! {
        a href="/admin" { "Home" }
        a href="/admin/articles" { "Articles" }
    }
}
