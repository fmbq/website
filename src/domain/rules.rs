use serde::Deserialize;
use sqlx::Acquire;
use std::sync::OnceLock;
use chrono::NaiveDate;
use crate::util::date_serialization::my_date_format;

static RULEBOOK_XML: &str = include_str!("../data/rules2021.xml");
static RULEBOOK: OnceLock<Rulebook> = OnceLock::new();

#[derive(Clone, Debug, Deserialize)]
pub struct Link {
    #[serde(rename = "$text")]
    pub text: String,
    #[serde(rename = "@uri")]
    pub uri: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TitlePage {
    pub title: String,
    pub subtitle: String,
    pub edition: String,
    pub copyright: String,
    pub website: Link,
    pub email: Link,
    #[serde(with = "my_date_format")]
    pub effective: NaiveDate,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SubTitlePage{
    pub section: Vec<String>,
}


#[derive(Clone, Debug, Deserialize)]
pub struct ListItem{
    #[serde(rename = "$text")]
    pub text: String,
    #[serde(default)]
    #[serde(rename = "section")]
    pub sections: Vec<ContentData>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ListOption{
    Unordered,
    Numeric,
    AlphaLowercase,
    AlphaUppercase,
}

#[derive(Clone, Debug, Deserialize)]
pub struct List{
    #[serde(rename = "@option")]
    pub option: Option<ListOption>,
    #[serde(rename = "item")]
    pub items: Vec<ListItem>,

}

#[derive(Clone, Debug, Deserialize)]
pub struct Note{
    #[serde(rename = "$text")]
    pub text: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Section{
    #[serde(rename = "$text")]
    text: String,        
}


#[derive(Clone, Debug, Deserialize)]
pub enum ContentData{
    #[serde(rename = "list")]
    List(List),
    #[serde(rename = "note")]
    Note(Note),
    #[serde(rename = "section")]
    Section(Section),
}

#[derive(Clone, Debug, Deserialize)]
pub enum RuleChild {
    #[serde(rename = "rule")]
    Rule(Rule),
    #[serde(rename = "list")]
    List(List),
    #[serde(rename = "note")]
    Note(Note),
    #[serde(rename = "section")]
    Section(Section),
}

#[derive(Clone, Debug, Deserialize)]
pub struct Rule{
    #[serde(rename = "@id")]
    pub id: u16,
    // header
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
    #[serde(rename = "@title")]
    pub title: Option<String>,

    #[serde(default)]
    #[serde(rename = "$value")]
    pub children: Vec<RuleChild>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Content{
    #[serde(rename = "rule")]
    pub rules: Vec<Rule>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Rulebook {
    #[serde(rename = "@year")]
    pub year: String,
    #[serde(rename = "@effective")]
    pub effective: String,

    pub titlepage: TitlePage,
    pub subtitlepage: SubTitlePage,
    pub contents: Content,
}

pub fn get_rulebook() -> &'static Rulebook {
    RULEBOOK.get_or_init(load)
}

fn load() -> Rulebook {
    quick_xml::de::from_str(RULEBOOK_XML).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
}
