use serde::Deserialize;
use sqlx::Acquire;
use std::sync::OnceLock;

static QUOTES_XML: &str = include_str!("../data/quotes.xml");
static ALL_QUOTES: OnceLock<Quotes> = OnceLock::new();

#[derive(Clone, Debug, Deserialize)]
pub struct QuoteSeason {
    id: u64,
    books: String,
    verses: u16,
    months: Vec<QuoteMonth>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct QuoteMonth {
    id: String,
    name: String,
    material: String,
    verses: u16,
    quotes: Vec<QuoteVerse>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct QuoteVerse{
    reference: String,
    verse: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Quotes {
    pub seasons: Vec<QuoteSeason>,
}

/// Get all quote seasons.
pub fn get_all() -> &'static [QuoteSeason] {
    ALL_QUOTES.get_or_init(load).seasons.as_slice()
}

fn load() -> Quotes {
    quick_xml::de::from_str(QUOTES_XML).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
}
