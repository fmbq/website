use super::copyright::copyright;
use maud::{html, Markup};

pub fn footer() -> Markup {
    html! {
        footer {
            .container {
                h4 { "Social Media" }
                p {
                    a target="_blank" href="https://www.facebook.com/fmbiblequizzing/" {
                        "Facebook"
                    }
                }

                (copyright())
            }
        }
    }
}
