use serde::Deserialize;
use sqlx::Acquire;
use std::sync::OnceLock;

static RULES_XML: &str = include_str!("../data/rules2021.xml");
static ALL_RULES: OnceLock<Rules> = OnceLock::new();

#[derive(Clone, Debug, Deserialize)]
pub struct RulesSeason {
    #[serde(rename = "@id")]
    pub id: u64,
    #[serde(rename = "@books")]
    pub books: String,
    #[serde(rename = "@verses")]
    pub verses: u16,
    pub months: Vec<RulesMonth>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RulesMonth{
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@material")]
    pub material: String,
    #[serde(rename = "@verses")]
    pub verses: u16,
    pub quotes: Vec<RulesVerse>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RulesVerse{
    #[serde(rename = "@reference")]
    pub reference: String,
    pub verse: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Rules {
    pub seasons: Vec<RulesSeason>,
}

/// Get all quote seasons.
pub fn get_all() -> &'static [RulesSeason] {
    ALL_RULES.get_or_init(load).seasons.as_slice()
}

fn load() -> Rules {
    quick_xml::de::from_str(RULES_XML).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
}
