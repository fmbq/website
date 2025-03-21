use super::images::SmartFigure;
use maud::{html, Markup, Render};

pub struct BigImageCard<'a> {
    pub title: &'a str,
    pub image_src: &'a str,
    pub body: Markup,
}

impl Render for BigImageCard<'_> {
    fn render(&self) -> Markup {
        html! {
            .big-image-card.full-width {
                h2 { (self.title) }

                .card-contents {
                    .left {
                        (SmartFigure {
                            image_src: self.image_src,
                            alt_text: self.title,
                            caption: None,
                        })
                    }
                    .right {
                        (self.body)
                    }
                }
            }
        }
    }
}
