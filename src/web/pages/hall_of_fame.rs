use crate::{
    domain::hall_of_fame::{get_halloffame, HOFInductee, HOFQuestion, HOFAnswer },
    web::components::layout::layout,
};

use maud::{html, Markup};

pub fn render() -> Markup {
    let hof = get_halloffame();
    let hof_image_folder = "./static/resources/photos/hall-of-fame/";
    layout(
        "Hall of Fame Members",
        html! {
            h1 { "Hall of Fame Members" }

            @for person in &hof.inductee {
                div {
                    h3 {(person.name)}
                    img src={(hof_image_folder)(person.image)} height="200px" {""}
                    br;
                    p {
                        "Inducted:" (person.inducted)
                        br;
                        "Participated:" (person.participated)
                        br;
                        "Church:" (person.church)
                        br;
                        "Location:" (person.city) "," (person.state)
                        br;
                    }

                    @for question in &person.questions {
                        p {
                            strong {(question.text)}
                            @for answer in &question.answers {
                                br;
                                (answer.text)
                            }
                        }
                    }
                }
            }
            p{}
            p{}
        },
    )
}
