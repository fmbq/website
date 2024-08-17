//! Markdown parsing and rendering based on pulldown-cmark with some added
//! features.

use crate::url;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use pulldown_cmark::{html, Event, LinkType, Options, Parser, Tag};
use sha1::{Digest, Sha1};
use std::iter;

/// Render a block of Markdown into HTML.
pub fn render_html(markdown: impl AsRef<str>) -> String {
    static CACHE: Lazy<DashMap<[u8; 20], String>> = Lazy::new(DashMap::new);

    let markdown = markdown.as_ref();
    let hash = Sha1::digest(markdown).into();

    if let Some(result) = CACHE.get(&hash) {
        return result.clone();
    }

    let options = Options::all();
    let parser = autolink(Parser::new_ext(markdown, options));

    fn render<'a>(parser: impl Iterator<Item = Event<'a>>) -> String {
        let mut html_buf = String::new();
        html::push_html(&mut html_buf, parser);

        html_buf
    }

    let result = render(parser);

    CACHE.insert(hash, result.clone());

    result
}

/// Detect URLs in plain text and transform them into links automatically.
fn autolink<'a>(mut events: impl Iterator<Item = Event<'a>>) -> impl Iterator<Item = Event<'a>> {
    let mut stack = Vec::new();
    let mut inside_code_block = false;
    let mut preparsed_count = 0;

    iter::from_fn(move || {
        let preparsed = if preparsed_count > 0 {
            preparsed_count -= 1;
            true
        } else {
            false
        };

        match stack.pop().or_else(|| events.next())? {
            Event::Text(text) if !preparsed && !inside_code_block => {
                if let Some((prefix, url, suffix)) = url::find(&text) {
                    let link = Tag::Link(LinkType::Autolink, url.clone().into(), "".into());

                    stack.push(Event::Text(suffix.into()));
                    stack.push(Event::End(link.clone()));
                    stack.push(Event::Text(url.into()));
                    stack.push(Event::Start(link.clone()));

                    preparsed_count += 3;

                    return Some(Event::Text(prefix.into()));
                } else {
                    Some(Event::Text(text))
                }
            }
            event @ Event::Start(Tag::CodeBlock(_)) => {
                inside_code_block = true;
                Some(event)
            }
            event @ Event::End(Tag::CodeBlock(_)) => {
                inside_code_block = false;
                Some(event)
            }
            event => Some(event),
        }
    })
}
