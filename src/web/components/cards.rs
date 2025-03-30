use super::images::SmartFigure;
use maud::{html, Markup, Render};

pub struct BigImageCard<'a> {
    pub title: &'a str,
    pub image_src: &'a str,
    pub alignment: ImageAlignment,
    pub body: Markup,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ImageAlignment {
    #[default]
    Left,
    Right,
}

impl Render for BigImageCard<'_> {
    fn render(&self) -> Markup {
        html! {
            .big-image-card {
                h2 { (self.title) }

                .card-contents .right[self.alignment == ImageAlignment::Right] {
                    .figure {
                        (SmartFigure {
                            image_src: self.image_src,
                            alt_text: self.title,
                            caption: None,
                        })
                    }
                    .body {
                        (self.body)
                    }
                }
            }
        }
    }
}
