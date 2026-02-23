use chrono::NaiveDate;
use serde::Deserialize;
use std::sync::OnceLock;

use crate::domain::seasons::get_current_season_year;

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
    #[serde(rename = "@start-date")]
    pub start_date: Option<NaiveDate>,
    #[serde(rename = "@end-date")]
    pub end_date: Option<NaiveDate>,
}

impl Finalsitem {
    pub fn event_date(&self) -> String {
        if let (Some(start), Some(end)) = (self.start_date, self.end_date) {
            format!("{} to {}", start, end)
        } else {
            " ".into()
        }
    }

    pub fn formatted_venue(&self) -> String {
        format!("{} in {}, {}", self.place, self.city, self.state)
    }
}


#[derive(Clone, Debug, Deserialize)]
pub struct Finals {

    #[serde(rename = "location")]
    pub finals: Vec<Finalsitem>,
}

impl Finals {
    // in the xml file, the year represents the year the finals ACTUALLY took place, not the season year
    // in our coding, we are using the season year, so we need to take that into account
    pub fn for_season_year(&self, year: u16) -> Option<&Finalsitem> {
        self.finals.iter().find(|f| f.year == year + 1)
    }
}
pub fn get_finals() -> &'static Finals {
    FINALS.get_or_init(load)
}

pub fn get_current_finalsitem() -> Option<&'static Finalsitem> {
    let year = get_current_season_year();
    get_finals().for_season_year(year)
}

fn load() -> Finals {
    quick_xml::de::from_str(FINALS_XML).unwrap()
}


#[cfg(test)]
mod tests {}
