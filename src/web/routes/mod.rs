use super::{pages, sse};
use crate::config::{Configuration, DeploymentEnvironment};
use ::time::{format_description::well_known::Rfc2822, OffsetDateTime};
use maud::Markup;
use poem::{
    handler,
    web::{sse::SSE, Data},
    IntoResponse,
};

pub mod admin;
pub mod article_images;
pub mod articles;
pub mod awards;
pub mod seasons;
pub mod css;
pub mod errors;
pub mod photos;
pub mod rules;
pub mod signup;

#[handler]
pub fn home() -> Markup {
    pages::home::render()
}

#[handler]
pub fn about() -> Markup {
    pages::about::render()
}

#[handler]
pub fn resources() -> Markup {
    pages::resources::render()
}

#[handler]
pub fn support_us() -> Markup {
    pages::support_us::render()
}

#[handler]
pub fn contacts() -> Markup {
    pages::contacts::render()
}

#[handler]
pub fn playground() -> Markup {
    pages::playground::render()
}

#[handler]
pub fn material() -> Markup {
    pages::material::render()
}

#[handler]
pub fn events() -> SSE {
    sse::subscribe()
}

#[handler]
pub fn time() -> impl IntoResponse {
    OffsetDateTime::now_utc().format(&Rfc2822).unwrap()
}

#[handler]
pub fn robots(Data(config): Data<&Configuration>) -> &'static str {
    match config.deployment_environment {
        DeploymentEnvironment::Testing => {
            "User-agent: *\n\
            Disallow: /"
        }
        DeploymentEnvironment::Production => {
            "User-agent: *\n\
            Disallow: /admin\n\
            Disallow: /static/\n\
            Disallow: /styles/\n\
            Disallow: /js/"
        }
    }
}
