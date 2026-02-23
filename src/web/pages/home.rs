use crate::web::components::layout::layout;
use maud::{html, Markup};

pub fn render() -> Markup {
    layout(
        "Free Methodist Bible Quizzing",
        html! {
            .hero.full-width.content-grid {
                h1 { "Free Methodist Bible Quizzing" }
            }

            .section.dark {
                p {
                    "Looking to start Bible Quizzing in your church?"
                    p {

                    a.button href="/get-started" {
                        "Get Started"
                    }
                }
                }
            }

            .section.dark {
                p {
                "INSTILL BIBLICAL TRUTH | ESTABLISH IN FAITH | PREPARE FOR SERVICE"
                }
            }

           .section.dark {
                p {
                    "Bible Quizzing is a Free Methodist Church ministry that encourages 6th through 12th grade students to
                    study and memorize Scripture through competition. A quizmaster asks questions in a series of
                    rounds where teams compete for the most points. Coaches help train the teams and take them to
                    monthly competitions from October to April. At the end of June, quiz team nationwide compete
                    in weeklong competition."
                }
            }

           h2 { "2024-2025 Season Information" }
           p {
                "Romans & James, quote list link, purchase link"
           }
           p {
               br{"2025 Quiz Finals is scheduled to take place on the west coast (tenatively Seattle Pacific University)"}
               br{"Sunday, June 29, 2025 - Thursday, July 3, 2025"}
           }
           h2 { "Free Methodist Denomination" }
        },
    )
}
