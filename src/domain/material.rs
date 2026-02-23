use serde::Deserialize;
use std::sync::OnceLock;

use crate::domain::seasons::get_current_season_year;

static MATERIAL_XML: &str = include_str!("../data/material.xml");
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

impl Materialitem {
    pub fn title(&self) -> String {
        format!("{}-{}", self.year, self.year + 1)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Material {
    #[serde(rename = "material")]
    pub materials: Vec<Materialitem>,
}

impl Material {
    pub fn for_season_year(&self, year: u16) -> Option<&Materialitem> {
        self.materials.iter().find(|m| m.year == year)
    }
}

pub fn get_material() -> &'static Material {
    MATERIAL.get_or_init(load)
}

pub fn get_current_material() -> Option<&'static Materialitem> {
    let year = get_current_season_year();
    get_material().for_season_year(year)
}

fn load() -> Material {
    quick_xml::de::from_str(MATERIAL_XML).unwrap()
}


#[cfg(test)]
mod tests {}
