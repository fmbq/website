//! Responses that can be used to trigger client-side actions for htmx.

use poem::{http::StatusCode, IntoResponse};

/// A response that triggers a full client-side refresh of the page.
///
/// Corresponds to the `HX-Refresh` header.
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

/// A response that triggers a client-side redirection to a new URL.
///
/// Corresponds to the `HX-Redirect` header.
#[derive(Clone, Debug)]
pub struct Redirect {
    url: String,
}

impl Redirect {
    /// Create a new redirect response instance to the specified URL.
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}

impl From<String> for Redirect {
    fn from(url: String) -> Self {
        Self::new(url)
    }
}

impl IntoResponse for Redirect {
    fn into_response(self) -> poem::Response {
        poem::Response::builder()
            .status(StatusCode::NO_CONTENT)
            .header("HX-Redirect", self.url)
            .finish()
    }
}
