use maud::{Markup, html};
use crate::components::layout::layout;

pub fn render() -> Markup {
    layout(
        "Free Methodist Bible Quizzing",
        html! {
            .hero.full-width {
                h1 { "Free Methodist Bible Quizzing" }
            }

            .section.dark {
                p {
                    "Looking to start Bible Quizzing in your church?"

                    a.button href="/get-started" {
                        "Get Started"
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

           .section.dark {
                h2 { "The Basics" }
                div.home-teams.full-width {                
                    p { 
                        "In FM Bible Quizzing, two teams of three persons compete against each other, and up to 
                        four persons may attempt to give a correct answer to the question."
                    }
                }
                div.home-qm.full-width {                
                    p { 
                        "Quizzes are facilitated by a quizmaster, who reads the questions and then determines whether 
                        the answers are correct or incorrect. Answers other than Quote questions do not have to be 
                        word-perfect to earn full points (20 points)."
                    }
                }
                div.home-questions.full-width {                
                    p { 
                        "Monthly quiz meets occur on Saturdays and include 6-8 rounds of quizzing. Each team round 
                        is made up of 15 questions worth 20 points each. Unless there is a tie, then a three-question 
                        overtime is used to break the tie. "
                    }
                }
                div.home-jumping.full-width {                
                    p { 
                        "In order to determine which quizzer gets to answer first, quizzers use jump seats. These are 
                        electronic seat pads connected to a display that registers the first four persons to jump, in order."
                    }
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
