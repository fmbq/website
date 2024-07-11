use crate::components::layout::layout;
use maud::{html, Markup};

pub fn render() -> Markup {
    layout(
        "Resources",
        html! {
            h1 { "Resources" }
 
            h2 { "Materials" }
            p {
                ul {
                    li {"2024 - 2025 Romans & James "
                        a href="/quotes/" {"Quote List"}
                    }
                    li {"2023 - 2024 Matthew 14 - 28, 1 Thessalonians, 2 Thessalonians"}
                    }                 
            }
            
            h2 { "Quote List" }
            p {                
                "The qoutBible Quizzing is a Free Methodist Church ministry that encourages 6th through 12th grade students to
                study and memorize Scripture through competition. A quizmaster asks questions in a series of 
                rounds where teams compete for the most points. Coaches help train the teams and take them to
                monthly competitions."
            }

            h2 { "Rules" }

            p {
                "Each year a specific portion of scripture is chosen and quizzers will
                study that text in monthly increments throughout the school year. The competition
                comes when quizzers are matched with other quizzers to see who can answer the most
                questions in a series of 15-question quiz rounds. Each question is worth 20 points
                and the team with the highest score wins!"
            }

            h2 { "Study Supplies" }
                         
                p { 
                    "In order to determine which quizzer gets to answer first, quizzers use jump seats. These are 
                    electronic seat pads connected to a display that registers the first four persons to jump, in order."
                }
          
            h2 { "Electronics" }
            p {
                "Teams are comprised of 2 to 5 quizzers from the same church or city. Teams practice
                together for a monthly competition against other teams from other churches. " 
            }

        },
    )
}
