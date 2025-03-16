use serde::Deserialize;
use std::sync::OnceLock;

static MARKELL_XML: &str = include_str!("../data/markell.xml");
static MARKELL: OnceLock<Markell> = OnceLock::new();

#[derive(Clone, Debug, Deserialize)]
pub struct MarkellWinner {
    #[serde(rename = "$text")]
    pub name: String,
    #[serde(rename = "@year")]
    pub year: u16,
    #[serde(rename = "@location")]
    pub location: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Markell {
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "bio")]
    pub bio: String,
    #[serde(rename = "winner")]
    pub winners: Vec<MarkellWinner>,
}


pub fn get_markell() -> &'static Markell {
    MARKELL.get_or_init(load)
}

fn load() -> Markell {
    quick_xml::de::from_str(MARKELL_XML).unwrap()
}


#[cfg(test)]
mod tests {}
