use crate::web::components::layout::layout;
use maud::{html, Markup};

pub fn render() -> Markup {
    layout(
        "Support Us",
        html! {
            h1 { "Support Us" }

            h2 { "Prayer" }

            p {
                "Support us through prayer."
            }

            h2 { "Financially" }

            p { "Did you know that Laura's income for her work with FM Bible Quizzing comes solely from     donations?  She does not receive any salary as an employee of the denomination, but must raise her own support for this work.  Please consider becoming a monthly or occasional supporter to fund this position."
            }

            br;
            a.button href="https://give.fmcusa.org/donation/df-fmbq-director" {"Give"}

            h2 { "Join our Newsletter" }

            p {
                "If you'd like to be added to the FMBQ email newsletter, send your email address to " a href="mailto:lchristn12@gmail.com" {"lchristn12@gmail.com"}
            }

            p{}
            p{}
        },
    )
}
