use serde::Deserialize;
use std::sync::OnceLock;

static QUOTES_XML: &str = include_str!("../data/quotes2024.xml");
static ALL_QUOTES: OnceLock<Quotes> = OnceLock::new();

#[derive(Clone, Debug, Deserialize)]
pub struct QuoteSeason {
    #[serde(rename = "@id")]
    pub id: u64,
    #[serde(rename = "@books")]
    pub books: String,
    #[serde(rename = "@verses")]
    pub verses: u16,
    pub month: Vec<QuoteMonth>,
}

impl QuoteSeason {
    /// Get the quote count for a season
    pub fn get_quote_count(&self) -> u16 {
        self.month
            .iter()
            .map(|month| month.quote.len())
            .sum::<usize>()
            .try_into()
            .unwrap()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct QuoteMonth {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@material")]
    pub material: String,
    #[serde(rename = "@verses")]
    pub verses: u16,
    pub quote: Vec<QuoteVerse>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct QuoteVerse{
    #[serde(rename = "@reference")]
    pub reference: String,
    #[serde(rename = "$text")]
    pub verse: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Quotes {
    pub season: Vec<QuoteSeason>,
}

pub fn get_season_by_id(season_id: u64) -> Option<&'static QuoteSeason> {
    get_all()
        .iter()
        .find(|season| season.id == season_id)
}

/// Get all quote seasons.
pub fn get_all() -> &'static [QuoteSeason] {
    ALL_QUOTES.get_or_init(load).season.as_slice()
}

fn load() -> Quotes {
    quick_xml::de::from_str(QUOTES_XML).unwrap()
}

#[cfg(test)]
mod tests {
}
