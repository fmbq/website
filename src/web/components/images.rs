use maud::{html, Markup, Render};

pub struct SmartFigure<'a> {
    pub image_src: &'a str,
    pub alt_text: &'a str,
    pub caption: Option<Markup>,
}

impl Render for SmartFigure<'_> {
    fn render(&self) -> Markup {
        html! {
            figure.smart {
                img.backdrop src=(self.image_src) role="presentation" {}

                img src=(self.image_src) alt=(self.alt_text) {}

                @if let Some(caption) = &self.caption {
                    figcaption { (caption) }
                }
            }
        }
    }
}
