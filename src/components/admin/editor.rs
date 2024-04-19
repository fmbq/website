use maud::{html, Markup};

pub fn editor() -> Markup {
    html! {
        div {
            div id="editorjs" {
            }
        }
        script src="/static/js/vendor/editorjs/2.29.1/editorjs.js" {}
        script src="https://cdn.jsdelivr.net/npm/@editorjs/header@latest" {}
        script {
            (include_str!("editor.js"))
        }
    }
}
