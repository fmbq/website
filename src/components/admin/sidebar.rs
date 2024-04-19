use maud::{html, Markup};

pub fn sidebar() -> Markup {
    html! {
        div.sidebar {
            a href="/admin/articles" { "Articles" }
        }
    }
}
