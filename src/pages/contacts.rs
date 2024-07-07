use crate::components::layout::layout;
use maud::{html, Markup};

pub fn render() -> Markup {
    layout(
        "Contacts",
        html! {
            h1 { "Contacts" }
            h2 { "Whos running this mess?" }

            p {
                "Bible Quizzing is a ministry that encourages 6th through 12th grade students to
                study and memorize Scripture through competition. Each year a specific portion of
                scripture is chosen (1 & 2 Corinthians, John, Romans & James) and quizzers will
                study that text in weekly increments throughout the school year. The competition
                comes when quizzers are matched with other quizzers to see who can answer the most
                questions in a series of 15-question quiz rounds. This is done on a weekly basis
                during Quiz practice, monthly at local tournaments, and annually at
                Conference/Regional quizzes and Quiz Finals."
            }

            p {
                "Free Methodist Bible Quizzing uses the NIV-2011 translation of the Holy Bible. Our
                current season (2021-2022) is over " b { "1&2 Timothy, Titus, Matthew 1-13" } "."
            }

            div {
                (crate::components::admin::editor::editor())
            }
        },
    )
}
