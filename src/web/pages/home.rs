use crate::{
    domain::{finals::get_current_finalsitem, material::get_current_material},
    web::components::layout::layout,
};
use maud::{html, Markup};

pub fn render() -> Markup {
    let material_this_season = get_current_material();
    let finals_this_season = get_current_finalsitem();
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

            @if let Some(material) = material_this_season {
                  h2 { (material.title()) " Season Information" }
                  p {
                    (material.books)
                    br;
                    a.button href={"/seasons/"(material.year)"/quotes"}{"Quote List"}
                    "  "
                    a.button href={"/resources#study-supplies"}{"Study Supplies"}
                  }
                  @if let Some(finals) = finals_this_season {
                    p {
                        br{"Quiz Finals will be at " (finals.formatted_venue())}
                        br{(finals.event_date())}
                    }
                  }

            } @else {
                p {"Season not found!"}
            }
        },
    )
}
