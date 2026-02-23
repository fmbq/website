use std::ops::Add;

use chrono::{Datelike, Utc};

pub fn get_current_season_year() -> u16 {
     let dt = Utc::now();
     // quiz seasons are Aug - July - the website uses the starting year as the season year (e.g. 2025-2026 is 2025)
     if dt.month() > 7 {
        dt.year().try_into().unwrap()
     }
     else {
         dt.year().add(-1).try_into().unwrap()
     }

}
