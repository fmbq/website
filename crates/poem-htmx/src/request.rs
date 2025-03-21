//! Request extractors that allow you to identify information about requests
//! triggered by htmx.

use poem::{FromRequest, Request, RequestBody, Result};

/// Checks if the request is an htmx request.
pub struct HxRequest(pub bool);

impl<'a> FromRequest<'a> for HxRequest {
    async fn from_request(request: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        let is_hx_request = request.headers().get("hx-request").is_some();

        Ok(HxRequest(is_hx_request))
    }
}

/// Checks if the request is an htmx request.
pub struct HxBoosted(pub bool);

impl<'a> FromRequest<'a> for HxBoosted {
    async fn from_request(request: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        let is_hx_request = request.headers().get("hx-boosted").is_some();

        Ok(HxBoosted(is_hx_request))
    }
}
