use crate::web::components::layout::layout;
use maud::{html, Markup};

pub fn render() -> Markup {
    layout(
        "Awards",
        html! {
            h1 { "Alpha Omega" }
            p { "add description here" }
            p {
                img src="/static/resources/photos/alpha-omega.jpg" height="400px" {""}
            }

            h1 { "Benson" }
            p { "add description here" }
            p {
                img src="/static/resources/photos/benson.jpg" height="400px" {""}
            }

            h1 { "Dave Markell Attitude in Excellence Award" }
            p { "The " a href="/markell" {"Dave Markell Attitude in Excellence Award"} " is given out annually at quiz finals." }
            
            
            h1 { "Hall of Fame" }
            p { "Beginning at the 2003 Bible Quiz Finals at Roberts Wesleyan College (on the 50th anniversary of Free Methodist Bible Quizzing), special recognition was given to former quizzers who not only did well in competition, but more importantly who applied what they learned and are currently living God-honoring lives in faithful service for the Lord." }

            h3 { "Purpose:" }
            p {
                ul {
                    li {"To connect former quizzers with the denominational program and honor their accomplishments and service in the Bible quiz ministry." }
                    li {"To record the blessings and benefits of Bible quizzing in order to encourage current quizzers and coaches." }
                    li { "To gather additional information about our Bible quizzing history and keep a record for future quizzers." }
                }
            }

            h3 { "Qualifications for Nominees:" }
            p {
                ul {
                    li { "Is a committed follower of Jesus Christ, active in church participation, and faithfully serving the Lord." }
                    li { "Has competed at least two years in Bible Quizzing, and competed well in competition." }
                    li { "Is at least 23 years of age." }
                    li { "MUST be able to be present at the Quiz Finals on the occasion of their induction." }
                    li { "Quiz leadership that never competed as teens might also be considered." }
                }
            }

            p { "Through the Prophet Isaiah, God said, \"As the rain and the snow come down from heaven, and do not return to it without watering the earth and making it bud and flourish, so that it yields seed for the sower and bread for the eater, so is My word that goes out from My mouth: It will not return to Me empty, but will accomplish what I desire, and achieve the purpose for which I sent it.\" (Isaiah 55:10-11)"
            }

            a href="/hall-of-fame" {"Hall of Fame Members"}

            p {}
            p {}
        },
    )
}
