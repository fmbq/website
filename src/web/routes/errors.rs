use crate::web::pages;
use poem::{error::NotFoundError, http::StatusCode, web::Html, IntoResponse};

pub async fn not_found(_: NotFoundError) -> impl IntoResponse {
    Html(pages::errors::not_found()).with_status(StatusCode::NOT_FOUND)
}
