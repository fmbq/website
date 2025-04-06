use serde::Deserialize;
use std::sync::OnceLock;

static FINALS_XML: &str = include_str!("../data/finals.xml");
static FINALS: OnceLock<Finals> = OnceLock::new();

#[derive(Clone, Debug, Deserialize)]
pub struct Finalsitem {
    #[serde(default)]
    #[serde(rename = "@place")]
    pub place: String,
    #[serde(rename = "@year")]
    pub year: u16,
    #[serde(default)]
    #[serde(rename = "@city")]
    pub city: String,
    #[serde(rename = "@state")]
    pub state: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Finals {

    #[serde(rename = "location")]
    pub finals: Vec<Finalsitem>,
}


pub fn get_finals() -> &'static Finals {
    FINALS.get_or_init(load)
}

fn load() -> Finals {
    quick_xml::de::from_str(FINALS_XML).unwrap()
}


#[cfg(test)]
mod tests {}
