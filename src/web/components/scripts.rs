use maud::{html, Markup};

pub fn scripts() -> Markup {
    html! {
        script src="/js/vendor/htmx/1.9.3/htmx.min.js" {}
        script src="/js/vendor/htmx/1.9.3/ext/sse.js" {}
        script src="/js/vendor/hyperscript/0.9.12/_hyperscript.min.js" {}
    }
}
