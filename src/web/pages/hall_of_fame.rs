use crate::{
    domain::hall_of_fame::get_halloffame,
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
            i { "Through the Prophet Isaiah, God said, \"As the rain and the snow come down from heaven, and do not return to it without watering the earth and making it bud and flourish, so that it yields seed for the sower and bread for the eater, so is My word that goes out from My mouth: It will not return to Me empty, but will accomplish what I desire, and achieve the purpose for which I sent it.\" (Isaiah 55:10-11)"
            }

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
