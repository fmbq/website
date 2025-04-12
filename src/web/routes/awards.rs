use maud::Markup;
use poem::{get, handler, IntoEndpoint, Route};

use crate::web::pages;


pub fn routes() -> impl IntoEndpoint {
    Route::new()
        .at("/", get(awards))
        .at("/finals", get(finals))
        .at("/alpha-omega", get(alpha_omega))
        .at("/individuals", get(individuals))
        .at("/benson", get(benson))
        .at("/dave-markell", get(dave_markell))
        .at("/spitshine", get(spitshine))
        .at("/hall-of-fame", get(hall_of_fame))
}

#[handler]
pub fn awards() -> Markup {
    pages::awards::render()
}

#[handler]
pub fn finals() -> Markup {
    pages::awards::finals::render()
}

#[handler]
pub fn alpha_omega() -> Markup {
    pages::awards::alpha_omega::render()
}

#[handler]
pub fn individuals() -> Markup {
    pages::awards::individuals::render()
}

#[handler]
pub fn benson() -> Markup {
    pages::awards::benson::render()
}

#[handler]
pub fn dave_markell() -> Markup {
    pages::awards::dave_markell::render()
}

#[handler]
pub fn spitshine() -> Markup {
    pages::awards::spitshine::render()
}

#[handler]
pub fn hall_of_fame() -> Markup {
    pages::awards::hall_of_fame::render()
}

