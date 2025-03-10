use crate::web::components::layout::layout;
use maud::{html, Markup};

pub fn render() -> Markup {
    layout(
        "Get Started",
        html! {
            h1 { "Get Started" }

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
        },
    )
}
