use serde::Deserialize;
use std::sync::OnceLock;

static FINALS_XML: &str = include_str!("../data/finals.xml");
static MATERIAL: OnceLock<Material> = OnceLock::new();

#[derive(Clone, Debug, Deserialize)]
pub struct Materialitem {
    #[serde(rename = "@books")]
    pub books: String,
    #[serde(rename = "@year")]
    pub year: u16,
    #[serde(default)]
    #[serde(rename = "@translation")]
    pub translation: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Material {

    #[serde(rename = "finals")]
    pub finals: Vec<Materialitem>,
}


pub fn get_finals() -> &'static Material {
    MATERIAL.get_or_init(load)
}

fn load() -> Material {
    quick_xml::de::from_str(FINALS_XML).unwrap()
}


#[cfg(test)]
mod tests {}
