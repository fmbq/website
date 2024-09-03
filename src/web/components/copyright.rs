use maud::{html, Markup};
use time::OffsetDateTime;

pub fn copyright() -> Markup {
    html! {
        p class="center copyright" {
            "© " (OffsetDateTime::now_utc().year()) " Free Methodist Bible Quizzing"
        }
    }
}
