use maud::{html, Markup, DOCTYPE};

/// Base layout for an HTML email.
pub fn email_layout(body: Markup) -> Markup {
    // Keep in mind that the flavor of HTML supported by email clients is *super
    // weird* and not at all HTML5.

    html! {
        (DOCTYPE)
        html lang="en-US" {
            head {
                meta http-equiv="Content-Type" content="text/html; charset=UTF-8";
                meta content="width=device-width, initial-scale=1.0" name="viewport";
            }
            body {
                (body)
            }
        }
    }
}
