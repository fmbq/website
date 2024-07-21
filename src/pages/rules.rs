use crate::{components::layout::layout, 
domain::rules::{get_rulebook, Rule}};

use maud::{html, Markup};
fn make_index_number(ids: &[u16]) -> String {
	"0.0.0.0".to_string()
}

fn render_rule(rule: &Rule, ids: &[u16]) -> Markup {
	let mut my_ids: Vec<u16> = ids.into();
	my_ids.push(rule.id);

	html! {	
		@if let Some(title) = &rule.title {
			div.rule_title { 
				h1 { (make_index_number(&my_ids)) }
				h2 { (title) }
			}
		}
		@else{
			// recurse rules/children 
			div.rule { 
				h1 { (make_index_number(&my_ids)) }
				h2 { "child markup from recursion"}
			}
		}

		hr;
	}
}

pub fn render() -> Markup {
    let rulebook = get_rulebook();
	dbg!(rulebook);
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
				div { (render_rule(rule, &[])) }
			}

        },
    )
}
