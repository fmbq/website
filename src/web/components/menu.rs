use maud::{html, Markup, Render};

pub enum Item<'a> {
    Leaf {
        name: &'a str,
        link: &'a str,
    },
    Branch {
        name: &'a str,
        link: &'a str,
        children: &'a [Item<'a>],
    },
}

/// Render a tree of menu items to HTML.
pub fn menu_tree(items: &[Item]) -> Markup {
    html! {
        ul.menu-tree {
            @for item in items {
                (item)
            }
        }
    }
}

impl Render for Item<'_> {
    fn render(&self) -> Markup {
        match self {
            Item::Leaf { name, link } => html! {
                li.menu-item {
                    a href=(link) { (name) }
                }
            },
            Item::Branch { name, link, children } => html! {
                li.menu-item {
                    a href=(link) { (name) }
                    (menu_tree(children))
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu() {
        let _ = Item::Branch {
            name: "Awards",
            link: "/awards",
            children: &[
                Item::Leaf {
                    name: "Individuals",
                    link: "/awards/individuals",
                },
                Item::Leaf {
                    name: "Hall of Fame",
                    link: "/awards/hall-of-fame",
                },
            ],
        };
    }
}
