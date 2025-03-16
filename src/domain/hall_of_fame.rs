use serde::Deserialize;
use std::sync::OnceLock;

static HOF_XML: &str = include_str!("../data/halloffame.xml");
static HALLOFFAME: OnceLock<HallOfFame> = OnceLock::new();

#[derive(Clone, Debug, Deserialize)]
pub struct HOFInductee {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@inducted")]
    pub inducted: u16,
    #[serde(rename = "@participated")]
    pub participated: String,
    #[serde(rename = "@church")]
    pub church: String,
    #[serde(rename = "@city")]
    pub city: String,
    #[serde(rename = "@state")]
    pub state: String,
    #[serde(rename = "@image")]
    pub image: String,
    #[serde(rename = "qa")]
    pub questions: Vec<HOFQuestion>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HOFQuestion {
    #[serde(rename = "@question")]
    pub text: String,
    #[serde(rename = "answer")]
    pub answers: Vec<HOFAnswer>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HOFAnswer {
    #[serde(rename = "$text")]
    pub text: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HallOfFame {
    pub inductee: Vec<HOFInductee>,
}


pub fn get_halloffame() -> &'static HallOfFame {
    HALLOFFAME.get_or_init(load)
}

fn load() -> HallOfFame {
    quick_xml::de::from_str(HOF_XML).unwrap()
}


#[cfg(test)]
mod tests {}
