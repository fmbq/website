use crate::web::components::layout::layout;
use maud::{html, Markup};

pub mod quotes;
pub mod finals;
pub mod finals_awards;
pub mod finals_news;
pub mod finals_registration;
pub mod finals_schedule;
pub mod finals_tournament;

pub fn render() -> Markup {
    layout(
        "Schedule",
        html! {


            h1 {"2024 - 2025 Romans & James"}
            h2 { "March" }
            h3 { "Romans 16, James 1 - 2"}

            p {
                ul {
                    li {"March 8, Greenville, IL" }
                    }
            }


            h2 { "April" }
            h3 { "James 3 - 5"}

            p {
                ul {
                    li {"April 12, Bedford, IN" }
                    }
            }

            h1 { "Finals" }
            p {
                div { "Something about finals."}
                a href="/seasons/finals" { "Finals"}
                br;
                "Link to past finals"
            }
        },
    )
}
