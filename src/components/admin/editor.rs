use maud::{html, Markup};

pub fn editor() -> Markup {
    html! {
        div {
            div id="editorjs" {
            }
        }
        script src="/js/vendor/editorjs/2.29.1/editorjs.js" {}
        script src="/js/vendor/editorjs-header/2.8.5/header.js" {}
        script {
            (include_str!("editor.js"))
        }
    }
}
