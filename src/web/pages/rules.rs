use crate::{
    web::components::layout::layout,
    domain::rules::{get_rulebook, ContentData, List, ListOption, Rule, RuleChild},
};

use maud::{html, Markup};

fn render_rule(rule: &Rule, ids: &[u16]) -> Markup {
    let mut my_ids: Vec<u16> = ids.to_vec();
    my_ids.push(rule.id);
    let id_string: String = my_ids
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(".");

    html! {
        @if let Some(title) = &rule.title {
            @if ids.is_empty() {
                div.rule_title { (id_string) }
                div.rule_title { (title) }
                hr;
            } @else {
                br;
                div.rule_subtitle { (id_string) }
                div.rule_subtitle { (title) }
            }
        } @else {
            //only print the rule
            @if let Some(t) = &rule.r#type {
                @if !t.eq_ignore_ascii_case("hidden") {
                    div.rule_normal { (id_string) }
                }
            } @else {
                div.rule_normal { (id_string) }
            }
        }

        @for child in &rule.children {
            @match child {
                RuleChild::Rule(r) => {
                    div { (render_rule(r, &my_ids)) }
                },
                RuleChild::List(l) => {
                    div { (render_list(l)) }
                },
                RuleChild::Note(n) => {
                    p { (n.text) }
                },
                RuleChild::Section(s) => {
                    div.rule_normal { (s.text) }
                }
            }
        }
    }
}

fn render_list(list: &List) -> Markup {
    html! {
        // need to parse the list type
        @if let Some(opt) = &list.option {
            @match opt {
                ListOption::Unordered => {
                    ul { (render_list_items(list)) }
                },
                ListOption::Numeric => {
                    ol { (render_list_items(list)) }
                },
                ListOption::AlphaLowercase => {
                    ol type="a" { (render_list_items(list)) }
                },
                ListOption::AlphaUppercase => {
                    ol type="A" { (render_list_items(list)) }
                }
            }
        }
    }
}

fn render_list_items(list: &List) -> Markup {
    html! {
        @for item in &list.items {
            li { (item.text) }
            @for section in &item.sections {
                @match section {
                    ContentData::List(l) => {
                        div { (render_list(l)) }
                    },
                    ContentData::Note(n) => {
                        p { (n.text) }
                    },
                    ContentData::Section(s) => {
                        p { (s.text) }
                    }
                }
            }
        }
    }
}

pub fn render() -> Markup {
    let rulebook = get_rulebook();
    let vec: Vec<u16> = Vec::new();
    //dbg!(rulebook);
    layout(
        "Rules",
        html! {
            h1 { (rulebook.titlepage.title) }

            h2 { (rulebook.titlepage.subtitle) }
            h3 { (rulebook.titlepage.edition) }

            p { (rulebook.titlepage.copyright) }
            br;
            div {
                a href=(rulebook.titlepage.website.uri) { (rulebook.titlepage.website.text) }
                " | "
                a href=(rulebook.titlepage.email.uri) { (rulebook.titlepage.email.text) }
            }
            br;
            div { "Effective " (rulebook.titlepage.effective.format("%v")) }
            br;

            hr;

            @for rule in &rulebook.contents.rules {
                div { (render_rule(rule, &vec)) }
            }

            p;
            p;

        },
    )
}
