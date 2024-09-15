//! Helpers for HTMX integration.

use poem::{http::StatusCode, IntoResponse};

/// A response that triggers a full refresh of the page.
#[derive(Clone, Debug)]
pub struct Refresh;

impl IntoResponse for Refresh {
    fn into_response(self) -> poem::Response {
        poem::Response::builder()
            .status(StatusCode::NO_CONTENT)
            .header("HX-Refresh", "true")
            .finish()
    }
}
