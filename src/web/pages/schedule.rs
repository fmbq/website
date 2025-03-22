use crate::web::components::layout::layout;
use maud::{html, Markup};

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
                a href="/finals" { "Finals"}
            }

            h1 { "The Study Cycle"}
            p{
                ul {
                    li { "Ruth, Luke 1-10" }
                    li { "Jonah, Luke 11-24"}
                    li { "1 & 2 Corinthians"}
                    li { "John"}
                    li { "Hebrews, 1 & 2 Peter, Jude"}
                    li { "1 & 2 Timothy, Titus, Matthew 1-13"}
                    li { "Matthew 14-28, 1 & 2 Thessalonians"}
                    li { "Romans & James"}
                    li { "Acts 1-14, Galatians, Colossians, Philemon"}
                    li { "Acts 15-28, Ephesians, Philippians"}
                    li { "1 John, 2 John, 3 John, Revelation"}
                }
            }
        },
    )
}
