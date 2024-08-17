use crate::components::layout::layout;
use maud::{html, Markup};

pub fn render() -> Markup {
    layout(
        "About",
        html! {
            h1 { "About" }

            h2 { "The Mission" }
            p {
                "INSTILL BIBLICAL TRUTH | ESTABLISH IN FAITH | PREPARE FOR SERVICE"
            }

            h2 { "The Players" }
            p {
                "Bible Quizzing is a Free Methodist Church ministry that encourages 6th through 12th grade students to
                study and memorize Scripture through competition. A quizmaster asks questions in a series of 
                rounds where teams compete for the most points. Coaches help train the teams and take them to
                monthly competitions."
            }

            h2 { "The Game" }

            p {
                "Each year a specific portion of scripture is chosen and quizzers will
                study that text in monthly increments throughout the school year. The competition
                comes when quizzers are matched with other quizzers to see who can answer the most
                questions in a series of 15-question quiz rounds. Each question is worth 20 points
                and the team with the highest score wins!"
            }

            h2 { "The Basics" }
            p {
                "In FM Bible Quizzing, two teams of three persons compete against each other, and up to four persons may attempt to give a correct answer to the question."
            }

            p {
                "Quizzes are facilitated by a quizmaster, who reads the questions and then determines whether the answers are correct or incorrect. Answers other than Quote questions do not have to be word-perfect to earn full points (20 points)."
            }
            p {
                "Monthly quiz meets occur on Saturdays and include 6-8 rounds of quizzing. Each team round is made up of 15 questions worth 20 points each. Unless there is a tie, then a three-question overtime is used to break the tie. "
            }
            p {
                "In order to determine which quizzer gets to answer first, quizzers use jump seats. These are electronic seat pads connected to a display that registers the first four persons to jump, in order."
            }

            h2 { "The Competition" }
            p {
                "Teams are comprised of 2 to 5 quizzers from the same church or city. Teams practice together for a monthly competition against other teams from other churches. "
            }

            h2 { "The Schedule" }
            p {
                "Monthly competitions run from October to April. Competitions a held a participating churches across the country. Online competitions are also available. A regional finals prep quiz is held in May. Finals is an annual week-long competition that occurs at the end of June. Teams start studying in September."
            }
            div.about-map { }

            h2 { "The Expenses" }
            h3 { "Season"}
            p {
                "A set of jump seats costs from $300 to $400.
                Each quizzer will pay a registration fee of $20 and purchase a portion for $2 to $20. 
                The monthly competitions are $10 to $15 per team with a $5 per person lunch cost. In addition,
                travel expenses may be necessary. Some churches offer overnight housing for free. Practice questions
                are available for purchase as well as other study aides." 
            }

            h3 { "Finals"}
            p {
                "The cost of quiz finals can be broken down to per person, per team, and travel. The per person cost is around $400. The per team cost is around $300. Travel costs vary based on the number attending and the distance from your church to the finals host school."
            }

            h3 { "Scholarships"}
            p {
                "For quizzers who attend finals, college scholarships are available.
                Scholarships are up to $20,000! The scholarships may change each year, so check the finals
                page for details on the current scholarships.
                The following schools have provided scholarhips: Corban University, Northwest University,
                Olivet Nazarene University, Greenville University, Mount Vernon Nazarene University, 
                Waynesburg University, Warner University, Huntington University, Seattle Pacific University,
                Central Christian College of Kansas, and Roberts Wesleyan University" 
            }
            div.about-scholarships { }
        },
    )
}
