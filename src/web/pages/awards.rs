use crate::web::components::{cards::BigImageCard, layout::layout};
use maud::{html, Markup};

pub fn render() -> Markup {
    layout(
        "Awards",
        html! {
            h1 { "Awards" }

            (BigImageCard {
                title: "Alpha Omega",
                image_src: "/static/resources/photos/awards/alpha-omega-2015.jpg",
                body: html! {
                    p { "The Alpha-Omega award is for STV teams that win at finals." }
                    p {
                        a href="/alphaomega" {"Past Winners"}
                    }
                },
            })

            (BigImageCard {
                title: "Benson",
                image_src: "/static/resources/photos/awards/benson-2015.jpg",
                body: html! {
                    p { "The Benson award was started in 2015 for YTV teams that win at finals." }
                },
            })

            (BigImageCard {
                title: "STV Individuals",
                image_src: "/static/resources/photos/awards/individuals-awards.jpg",
                body: html! {
                    p { "Scholarships are awarded to the top finishers in STV." }
                },
            })

            (BigImageCard {
                title: "Dave Markell Attitude in Excellence Award",
                image_src: "https://placeholder.pics/svg/300/DEDEDE/555555/Dave%20Markell",
                body: html! {
                    p { "Each year at Quiz Finals, a graduating senior is honored for their excellence in attitude. This award is named in honor of Dave Markell. Rev. David Markell served as the Director of Youth Ministries during the 1980's, before his untimely death from leukemia. His leadership and advocacy of quizzing lives on through the lives of hundreds of quizzers, officials and coaches to whom he ministered. His attitude, even as he faced death, was always one of calm, wisdom and strength. His family has consented to the use of his name to honor the quizzer who most exemplifies the life of Christ through their attitudes and behavior in quizzing."}
                    p{"Directors, coaches and leaders are invited to nominate a graduating senior who will be in attendance at Quiz Finals."}
                    br {a href="/markell" {"Dave Markell Attitude in Excellence Award Recipients"}}
                },
            })

            (BigImageCard {
                title: "Spitshine Award",
                image_src: "/static/resources/photos/awards/spitshine-2011.jpg",
                body: html! {
                    p { "This is our award for the best uniform. Quizmasters, Conference Directors and Regional Directors vote for their favorite look. This does not have to be a T-shirt design (but it can be). Team names and individual names are not required elements. Teams are encouraged to “look sharp” in whatever way they choose to interpret that. An eye-catching, uniform Look may include elements like hats, vests, suspenders, handkerchiefs, footwear, ect. Be creative! Props should not distract or interfere with normal quizzing." }
                },
            })

            (BigImageCard {
                title: "Hall of Fame",
                image_src: "/static/resources/photos/awards/hall-of-fame.jpg",
                body: html! {
                    p { "Beginning at the 2003 Bible Quiz Finals at Roberts Wesleyan College (on the 50th anniversary of Free Methodist Bible Quizzing), special recognition was given to former quizzers who not only did well in competition, but more importantly who applied what they learned and are currently living God-honoring lives in faithful service for the Lord." }
                    p { strong {"Purpose:"} }
                    ul {
                        li {"To connect former quizzers with the denominational program and honor their accomplishments and service in the Bible quiz ministry." }
                        li {"To record the blessings and benefits of Bible quizzing in order to encourage current quizzers and coaches." }
                        li { "To gather additional information about our Bible quizzing history and keep a record for future quizzers." }
                    }

                    p { strong { "Qualifications for Nominees:" } }
                    ul {
                        li { "Is a committed follower of Jesus Christ, active in church participation, and faithfully serving the Lord." }
                        li { "Has competed at least two years in Bible Quizzing, and competed well in competition." }
                        li { "Is at least 23 years of age." }
                        li { "MUST be able to be present at the Quiz Finals on the occasion of their induction." }
                        li { "Quiz leadership that never competed as teens might also be considered." }
                    }

                    a href="/hall-of-fame" {"Hall of Fame Members"}
                },
            })

            p {}
            p {}
        },
    )
}
